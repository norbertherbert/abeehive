// use std::fmt;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;
use gloo::console::log;

use abeehive::components::my_input::MyInput;
use abeehive::components::my_hex_input::MyHexInput;
use abeehive::components::my_optional_input::MyOptionalInput;
use abeehive::components::my_select::MySelect;
// use abeehive::components::my_select1::MySelect1;
use abeehive::components::my_bitmap::MyBitmap;
use abeehive::components::my_transmit_strat_custom::MyTransmitStratCustom;

use js_sys::{Function, Promise};


// use abeehive::js::flowbite::initFlowbite;

use abeehive::params::{
    param_comp_constants::CONF_PARAMS,
    param_values::ParamValues,
    param_id::ParamId,
    param_options::TransmitStratOption,
    param_values::ValueUpdateData,
};


#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window.__TAURI__.event"], js_name = "listen")]
    fn listen_(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> Promise;

}





#[derive(Serialize, Deserialize)]
struct GreetArgs {
    param_values: ParamValues,
}

#[derive(Serialize, Deserialize)]
struct SaveAsArgs {
    param_values: ParamValues,
}




#[function_component(App)]
pub fn app() -> Html {

    // let param_values_ref = use_mut_ref(|| {
    //     let param_values: ParamValues = Default::default();
    //     param_values
    // });

    let force_update = use_force_update(); 

    // let state = use_state(|| {
    //     *param_values_ref.borrow()
    // });
    
    let (state, state_dispatch) = use_store::<ParamValues>();

    let greet_msg = use_state(|| String::new());

    let param_value_changed: Callback<ValueUpdateData> = {

        // let param_values_ref = param_values_ref.clone();
        let force_update = force_update.clone();

        let state_dispatch = state_dispatch.clone();
        
        Callback::from(move |value_update_data: ValueUpdateData| {

            match value_update_data.param_id {

                ParamId::Mode => { 
                    // param_values_ref.borrow_mut().mode = value_update_data.new_value;
                    state_dispatch.reduce_mut(|state| state.mode = value_update_data.new_value );
                },
                ParamId::UlPeriod => { 
                    // param_values_ref.borrow_mut().ul_period = value_update_data.new_value; 
                    state_dispatch.reduce_mut(|state| state.ul_period = value_update_data.new_value );
                },
                ParamId::LoraPeriod => { 
                    // param_values_ref.borrow_mut().lora_period = value_update_data.new_value; 
                    state_dispatch.reduce_mut(|state| state.lora_period = value_update_data.new_value );
                },
                ParamId::PeriodicPosPeriod => { 
                    // param_values_ref.borrow_mut().periodic_pos_period = value_update_data.new_value;
                    state_dispatch.reduce_mut(|state| state.periodic_pos_period = value_update_data.new_value ); 
                },
                ParamId::GeolocSensor => { 
                    // param_values_ref.borrow_mut().geoloc_sensor = value_update_data.new_value; 
                    state_dispatch.reduce_mut(|state| state.geoloc_sensor = value_update_data.new_value );
                },
                ParamId::GeolocMethod => { 
                    // param_values_ref.borrow_mut().geoloc_method = value_update_data.new_value; 
                    state_dispatch.reduce_mut(|state| state.geoloc_method = value_update_data.new_value );
                },
                ParamId::TransmitStrat => { 
                    // param_values_ref.borrow_mut().transmit_strat = value_update_data.new_value;
                    state_dispatch.reduce_mut(|state| state.transmit_strat = value_update_data.new_value );


                    force_update.force_update();
                    // state.set(*param_values_ref.borrow()); 
                },
                ParamId::TransmitStratCustom => { 
                    // param_values_ref.borrow_mut().transmit_strat_custom = value_update_data.new_value; 
                    state_dispatch.reduce_mut(|state| state.transmit_strat_custom = value_update_data.new_value );
                },
                ParamId::ConfigFlags => { 
                    // param_values_ref.borrow_mut().config_flags = value_update_data.new_value;
                    state_dispatch.reduce_mut(|state| state.config_flags = value_update_data.new_value );
                },

            };

        })
    };


    let on_submit = {

        // let param_values_ref = param_values_ref.clone();
        // let state = state.clone();
        let greet_msg = greet_msg.clone();

        let state = state.clone();
        // let state_dispatch = state_dispatch.clone();

        Callback::from(move |_: MouseEvent| {

            let greet_msg = greet_msg.clone();

            // let x: ParamValues = *param_values_ref.borrow();
            
            let x = *state;

            let args = to_value( &GreetArgs{ param_values: x.clone() } ).unwrap();

            spawn_local(async move {

                let new_msg = invoke("greet", args).await.as_string().unwrap();
                log!(&new_msg);
                greet_msg.set(new_msg);

            });
            
            // state_dispatch.set(*param_values_ref.borrow());

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
        // let param_values_ref = param_values_ref.clone();
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            // let param_values: ParamValues = (*param_values_ref.borrow()).clone();
            let param_values: ParamValues = (*state).clone();
            let args = to_value( &GreetArgs{ param_values } ).unwrap();
            spawn_local(async move {
                invoke("save_as", args).await; //.as_string().unwrap();
            });
        })
    };


    #[derive(Deserialize, Debug)]
    struct FileOpenEvent {
        // payload: String,
        payload: ParamValues,
    }


    {
        // let force_update = force_update.clone();
        // let param_values_ref = param_values_ref.clone();
        let state_dispatch = state_dispatch.clone();
        use_effect( move || {

            let file_open_closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {

                let file_open_event: FileOpenEvent = serde_wasm_bindgen::from_value(raw).unwrap();
                log!(format!("{}", &file_open_event.payload));

                // param_values_ref.replace(file_open_event.payload);
                state_dispatch.set(file_open_event.payload);

                // force_update.force_update();

            });

            let unlisten = listen_("FileOpen", &file_open_closure);
            let listener = (unlisten, file_open_closure);
            || {
                    let promise = listener.0.clone();
                    spawn_local(async move {
                        let unlisten: Function = wasm_bindgen_futures::JsFuture::from(promise)
                            .await
                            .unwrap()
                            .into();
                        unlisten.call0(&JsValue::undefined()).unwrap();
                    });
                    drop(listener);
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

                        // value={param_values_ref.borrow().mode}
                        value={state.mode}

                        handle_onchange={param_value_changed.clone()}
                    />

                    <MyInput
                        id={CONF_PARAMS.ul_period.id}
                        label={CONF_PARAMS.ul_period.label}
                        description={CONF_PARAMS.ul_period.description}

                        // value={param_values_ref.borrow().ul_period}
                        value={state.ul_period}

                        valid_range={CONF_PARAMS.ul_period.valid_range}
                        handle_onchange={param_value_changed.clone()}
                    />

                    <MyHexInput
                        id={CONF_PARAMS.lora_period.id}
                        label={CONF_PARAMS.lora_period.label}
                        description={CONF_PARAMS.lora_period.description}

                        // value={param_values_ref.borrow().lora_period}
                        value={state.lora_period}

                        valid_range={CONF_PARAMS.lora_period.valid_range}
                        handle_onchange={param_value_changed.clone()}
                    />

                    <MyOptionalInput
                        id={CONF_PARAMS.periodic_pos_period.id}
                        label={CONF_PARAMS.periodic_pos_period.label}
                        description={CONF_PARAMS.periodic_pos_period.description}

                        // value={param_values_ref.borrow().periodic_pos_period}
                        value={state.periodic_pos_period}

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

                        // value={param_values_ref.borrow().geoloc_sensor}
                        value={state.geoloc_sensor}

                        handle_onchange={param_value_changed.clone()}
                    />

                    // <MySelect1<tauri_app_ui::params::param_options1::GeolocMethodOption1>
                    //     id={CONF_PARAMS.geoloc_method.id}
                    //     label={CONF_PARAMS.geoloc_method.label}
                    //     description={CONF_PARAMS.geoloc_method.description}
                    //     select_options={tauri_app_ui::params::param_options1::GeolocMethodOption1::VARIANTS}
                    //     value={param_values_ref.borrow().geoloc_method}
                    //     handle_onchange={param_value_changed.clone()}
                    // />




                    <MySelect
                        id={CONF_PARAMS.geoloc_method.id}
                        label={CONF_PARAMS.geoloc_method.label}
                        description={CONF_PARAMS.geoloc_method.description}
                        select_options={CONF_PARAMS.geoloc_method.options}

                        // value={param_values_ref.borrow().geoloc_method}
                        value={state.geoloc_method}

                        handle_onchange={param_value_changed.clone()}
                    />

                    <div>
                        <MySelect
                            id={CONF_PARAMS.transmit_strat.id}
                            label={CONF_PARAMS.transmit_strat.label}
                            description={CONF_PARAMS.transmit_strat.description}
                            select_options={CONF_PARAMS.transmit_strat.options}

                            // value={param_values_ref.borrow().transmit_strat}
                            value={state.transmit_strat}

                            handle_onchange={param_value_changed.clone()}
                        />
                        <div 
                            // hidden={ param_values_ref.borrow().transmit_strat != TransmitStratOption::CUSTOM.value }
                            hidden={ state.transmit_strat != TransmitStratOption::CUSTOM.value }
                            class={"ml-5 mt-2"}
                        >
                            <MyTransmitStratCustom 
                                id={CONF_PARAMS.transmit_strat_custom.id}
                                label={CONF_PARAMS.transmit_strat_custom.label}
                                description={CONF_PARAMS.transmit_strat_custom.description}
                                items={CONF_PARAMS.transmit_strat_custom.bits}

                                // value={param_values_ref.borrow().transmit_strat_custom}
                                value={state.transmit_strat_custom}

                                handle_onchange={param_value_changed.clone()}
                            />
                        </div>
                    </div>

                    <MyBitmap 
                        id={CONF_PARAMS.config_flags.id}
                        label={CONF_PARAMS.config_flags.label}
                        description={CONF_PARAMS.config_flags.description}
                        items={CONF_PARAMS.config_flags.bits}

                        // value={param_values_ref.borrow().config_flags}
                        value={state.config_flags}

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
