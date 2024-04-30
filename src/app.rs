// use std::fmt;
// use yewdux::prelude::*;
use std::ops::Deref;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use gloo::console::log;
use js_sys::{Function, Promise};

// use abeehive::js::flowbite::initFlowbite;
use abeehive::components::my_input::MyInput;
use abeehive::components::my_hex_input::MyHexInput;
use abeehive::components::my_optional_input::MyOptionalInput;
use abeehive::components::my_select::MySelect;
// use abeehive::components::my_select1::MySelect1;
use abeehive::components::my_bitmap::MyBitmap;
use abeehive::components::my_transmit_strat_custom::MyTransmitStratCustom;
use abeehive::params::param_values::ParamValue;
use abeehive::params::{
    param_comp_constants::CONF_PARAMS,
    param_id::ParamId,
    param_options::TransmitStratOption,
    param_values::{ParamValues, ValueUpdateData},
};


#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window.__TAURI__.event"], js_name = "listen")]
    fn listen_(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> Promise;

}




#[derive(Serialize, Deserialize)]
struct SaveArgs {
    param_values: ParamValues,
}

#[function_component(App)]
pub fn app() -> Html {


    let force_update = use_force_update(); 
    
    let state = use_state(|| {
        let param_values: ParamValues = Default::default();
        param_values
    });
    
    let greet_msg = use_state(|| String::new());

    let param_value_changed: Callback<ValueUpdateData> = {

        let force_update = force_update.clone();
        let state = state.clone();
        
        Callback::from(move |value_update_data: ValueUpdateData| {

            match value_update_data.param_id {

                ParamId::Mode => { 
                    state.set(ParamValues{mode: value_update_data.new_param_value, ..state.deref().clone()});
                },
                ParamId::UlPeriod => { 
                    state.set(ParamValues{ul_period: value_update_data.new_param_value, ..state.deref().clone()});
                },
                ParamId::LoraPeriod => { 
                    state.set(ParamValues{lora_period: value_update_data.new_param_value, ..state.deref().clone()});
                },
                ParamId::PeriodicPosPeriod => { 
                    state.set(ParamValues{periodic_pos_period: value_update_data.new_param_value, ..state.deref().clone()});
                },
                ParamId::GeolocSensor => { 
                    state.set(ParamValues{geoloc_sensor: value_update_data.new_param_value, ..state.deref().clone()});
                },
                ParamId::GeolocMethod => { 
                    state.set(ParamValues{geoloc_method: value_update_data.new_param_value, ..state.deref().clone()});
                },
                ParamId::TransmitStrat => { 
                    state.set(ParamValues{transmit_strat: value_update_data.new_param_value, ..state.deref().clone()});
                    force_update.force_update();
                },
                ParamId::TransmitStratCustom => { 
                    state.set(ParamValues{transmit_strat_custom: value_update_data.new_param_value, ..state.deref().clone()});
                },
                ParamId::ConfigFlags => { 
                    state.set(ParamValues{config_flags: value_update_data.new_param_value, ..state.deref().clone()});
                },

            };

        })
    };


    let on_submit = {
        let greet_msg = greet_msg.clone();
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            let serialized_state = toml::to_string(&*state).unwrap();
            greet_msg.set(serialized_state);
        })
    };


    let on_open = {
        Callback::from(move |_: MouseEvent| {
            let args = to_value( &() ).unwrap();
            spawn_local(async move {
                invoke("open", args).await; // .as_string().unwrap();
            });
        })
    };

    let on_save_as = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            let param_values: ParamValues = (*state).clone();
            let args = to_value( &SaveArgs{ param_values } ).unwrap();
            spawn_local(async move {
                invoke("save_as", args).await; //.as_string().unwrap();
            });
        })
    };


    #[derive(Deserialize, Debug)]
    struct MenuClickEvent {
        payload: (String, String),
    }


    {

        let state = state.clone();

        use_effect( move || {

            let menu_click_closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
                let file_open_event: MenuClickEvent = serde_wasm_bindgen::from_value(raw).unwrap();

                match file_open_event.payload.0.as_str() {
                    "FileOpen" => {
                        log!(format!("{}", &file_open_event.payload.0));
                        let param_values: ParamValues = toml::from_str(&file_open_event.payload.1).unwrap();
                        dbg!(&param_values);
                        // state_dispatch.set(param_values);
                        state.set(param_values);
                    },
                    "FileSave" => {
                        log!(format!("{}", &file_open_event.payload.0));

                        let param_values: ParamValues = (*state).clone();
                        let args = to_value( &SaveArgs{ param_values } ).unwrap();
                        spawn_local(async move {
                            invoke("save_as", args).await; //.as_string().unwrap();
                        });

                    },
                    _ => {},
                }

            });
            let unlisten_menu_click = listen_("MenuClick", &menu_click_closure);


            let listeners = (unlisten_menu_click, menu_click_closure);
            || {
                    let promise_0 = listeners.0.clone();
                    spawn_local(async move {
                        let unlisten_0: Function = wasm_bindgen_futures::JsFuture::from(promise_0)
                            .await
                            .unwrap()
                            .into();
                        unlisten_0.call0(&JsValue::undefined()).unwrap();
                    });
                    drop(listeners);
            }

        });
    }

    html! {
        <main class="m-5">
            <div class="m-5">
                <div class="grid gap-5 mb-5 md:grid-cols-2">

                    <MySelect
                        id={CONF_PARAMS.mode.id}
                        label={CONF_PARAMS.mode.label}
                        description={CONF_PARAMS.mode.description}
                        select_options={CONF_PARAMS.mode.options}
                        value={state.mode.clone()}
                        handle_onchange={param_value_changed.clone()}
                    />

                    <MyInput
                        id={CONF_PARAMS.ul_period.id}
                        label={CONF_PARAMS.ul_period.label}
                        description={CONF_PARAMS.ul_period.description}
                        value={state.ul_period.clone()}
                        valid_range={CONF_PARAMS.ul_period.valid_range}
                        handle_onchange={param_value_changed.clone()}
                    />

                    <MyHexInput
                        id={CONF_PARAMS.lora_period.id}
                        label={CONF_PARAMS.lora_period.label}
                        description={CONF_PARAMS.lora_period.description}
                        value={state.lora_period.clone()}
                        valid_range={CONF_PARAMS.lora_period.valid_range}
                        handle_onchange={param_value_changed.clone()}
                    />

                    <MyOptionalInput
                        id={CONF_PARAMS.periodic_pos_period.id}
                        label={CONF_PARAMS.periodic_pos_period.label}
                        description={CONF_PARAMS.periodic_pos_period.description}
                        value={state.periodic_pos_period.clone()}
                        disabled_value={CONF_PARAMS.periodic_pos_period.disabled_value}
                        default_value={CONF_PARAMS.periodic_pos_period.default_value}
                        valid_range={CONF_PARAMS.periodic_pos_period.valid_range}
                        handle_onchange={param_value_changed.clone()}
                    />

                    <MySelect
                        id={CONF_PARAMS.geoloc_sensor.id}
                        label={CONF_PARAMS.geoloc_sensor.label}
                        description={CONF_PARAMS.geoloc_sensor.description}
                        select_options={CONF_PARAMS.geoloc_sensor.options}
                        value={state.geoloc_sensor.clone()}
                        handle_onchange={param_value_changed.clone()}
                    />

                    // // <MySelect1<tauri_app_ui::params::param_options1::GeolocMethodOption1>
                    // //     id={CONF_PARAMS.geoloc_method.id}
                    // //     label={CONF_PARAMS.geoloc_method.label}
                    // //     description={CONF_PARAMS.geoloc_method.description}
                    // //     select_options={tauri_app_ui::params::param_options1::GeolocMethodOption1::VARIANTS}
                    // //     value={param_values_ref.borrow().geoloc_method}
                    // //     handle_onchange={param_value_changed.clone()}
                    // // />

                    <MySelect
                        id={CONF_PARAMS.geoloc_method.id}
                        label={CONF_PARAMS.geoloc_method.label}
                        description={CONF_PARAMS.geoloc_method.description}
                        select_options={CONF_PARAMS.geoloc_method.options}
                        value={state.geoloc_method.clone()}
                        handle_onchange={param_value_changed.clone()}
                    />

                    <div>
                        <MySelect
                            id={CONF_PARAMS.transmit_strat.id}
                            label={CONF_PARAMS.transmit_strat.label}
                            description={CONF_PARAMS.transmit_strat.description}
                            select_options={CONF_PARAMS.transmit_strat.options}
                            value={state.transmit_strat.clone()}
                            handle_onchange={param_value_changed.clone()}
                        />
                        <div 
                            hidden={ state.transmit_strat.clone() != ParamValue::Valid(TransmitStratOption::CUSTOM.value) }
                            class={"ml-5 mt-2"}
                        >
                            <MyTransmitStratCustom 
                                id={CONF_PARAMS.transmit_strat_custom.id}
                                label={CONF_PARAMS.transmit_strat_custom.label}
                                description={CONF_PARAMS.transmit_strat_custom.description}
                                items={CONF_PARAMS.transmit_strat_custom.bits}
                                value={state.transmit_strat_custom.clone()}
                                handle_onchange={param_value_changed.clone()}
                            />
                        </div>
                    </div>

                    <MyBitmap 
                        id={CONF_PARAMS.config_flags.id}
                        label={CONF_PARAMS.config_flags.label}
                        description={CONF_PARAMS.config_flags.description}
                        items={CONF_PARAMS.config_flags.bits}
                        value={state.config_flags.clone()}
                        handle_onchange={param_value_changed.clone()}
                    />

                    <button
                        id="btn-submit" 
                        data-tooltip-target="submit-tooltip" 
                        class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        onclick={on_submit.clone()}
                    >
                        { "Submit" }
                    </button>
                    <div 
                        id="submit-tooltip" 
                        role="tooltip" 
                        class="absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-white transition-opacity duration-300 bg-gray-900 rounded-lg shadow-sm opacity-0 tooltip dark:bg-gray-700"
                    >
                        {"Click to submit the form!"}
                        <div class="tooltip-arrow" data-popper-arrow="true"></div>
                    </div>

                    <button
                        id="btn-save-as" 
                        class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        onclick={on_open.clone()}
                    >
                        { "Open" }
                    </button>
                    <button
                        id="btn-save-as" 
                        class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        onclick={on_save_as.clone()}
                    >
                        { "Save As..." }
                    </button>

                </div>
            </div>

            // <pre>{ &*state.to_string() }</pre>
            <pre>{ &*greet_msg }</pre>

        </main>
    }
}
