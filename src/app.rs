use abeehive::prm::val::PrmVVals;
// use std::fmt;
// use yewdux::prelude::*;
use gloo::console::log;
use js_sys::{Function, Promise};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
// use std::ops::Deref;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

// use abeehive::js::flowbite::initFlowbite;
use abeehive::components::RadixDisp;
// use abeehive::components::my_hex_input::MyHexInput;
use abeehive::components::my_input::MyInput;
use abeehive::components::my_optional_input::MyOptionalInput;
use abeehive::components::my_select::MySelect;
use abeehive::components::my_bitmap::MyBitmap;
// use abeehive::components::my_transmit_strat_custom::MyTransmitStratCustom;
// use abeehive::params::param_values::ParamValue;
use abeehive::params::param_values::{
    ParamValues, 
    // ValueUpdateData
};

use abeehive::prm::dat::*;

use abeehive::components::modal::Modal;
use abeehive::components::navbar::{Navbar, NavbarAction};
use abeehive::components::select_usb_port::SelectUsbPort;

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

#[derive(Serialize, Deserialize)]
struct GetConfigUsbArgs {
    port: String,
}

#[derive(Serialize, Deserialize)]
struct SaveConfigToUsbArgs {
    cli_cmds: Vec<String>,
    port: String,
}


#[derive(Debug, PartialEq)]
pub enum Msg {
    ParamValueChanged(u8, String),
    // Submit,
    // Open,
    // SaveAs,
    // GetConfigUsbModalToggle,
    // SaveConfigUsbModalToggle,
    // GetSerialPortsForGet,
    // GetSerialPortsForSave,
    // SelectedUsbPortChanged(String),
    // GetConfigUsb,
    // SaveConfigUsb
}

#[derive(Default, Debug)]
pub struct BeeQueenApp {
    vvals: PrmVVals,
    // get_serialport_modal_is_visible: bool,
    // save_serialport_modal_is_visible: bool,
    // serial_ports: Vec<String>,
    // selected_serial_port: String,
    // greet_msg: String,
}

impl Component for BeeQueenApp {
    type Message = Msg;
    type Properties = ();
  
    fn create(_ctx: &Context<Self>) -> Self {
        BeeQueenApp::default()
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <>
                <main class="pt-14">
                    <div class="m-7">
                        <div class="grid gap-5 mb-5 md:grid-cols-2">

                            <MySelect
                                id={MODE.id}
                                label={MODE.label}
                                description={MODE.description}
                                select_options={MODE.distinct_vals}

                                vval={
                                    self.vvals.get_by_id(MODE.id)
                                    .expect("id is always valid")
                                    .expect("id always exists")
                                    .clone()
                                }
                                handle_onchange = {
                                    ctx.link().callback(move |(id, txt)| {
                                        Msg::ParamValueChanged(id, txt)
                                    })
                                }
                            />

                            <MyInput
                                id = { UL_PERIOD.id }
                                label = { UL_PERIOD.label }
                                description = { UL_PERIOD.description }
                                radix_disp = { RadixDisp::Dec }
                                vval={
                                    self.vvals.get_by_id(UL_PERIOD.id)
                                    .expect("id is always valid")
                                    .expect("id always exists")
                                    .clone()
                                }
                                handle_onchange = {
                                    ctx.link().callback(move |(id, txt)| {
                                        Msg::ParamValueChanged(id, txt)
                                    })
                                }
                            />

                            <MyInput
                                id = { LORA_PERIOD.id }
                                label = { LORA_PERIOD.label }
                                description = { LORA_PERIOD.description }
                                radix_disp = { RadixDisp::Hex }
                                vval={
                                    self.vvals.get_by_id(LORA_PERIOD.id)
                                    .expect("id is always valid")
                                    .expect("id always exists")
                                    .clone()
                                }
                                handle_onchange = {
                                    ctx.link().callback(move |(id, txt)| {
                                        Msg::ParamValueChanged(id, txt)
                                    })
                                }
                            />

                            <MyOptionalInput
                                id = { PERIODIC_POS_PERIOD.id }
                                label = { PERIODIC_POS_PERIOD.label }
                                description = {PERIODIC_POS_PERIOD.description }
                                disabled_value = { PERIODIC_POS_PERIOD.disabled_val }
                                default_value = { PERIODIC_POS_PERIOD.default_val }
                                radix_disp = { RadixDisp::Dec }
                                vval={
                                    self.vvals.get_by_id(PERIODIC_POS_PERIOD.id)
                                    .expect("id is always valid")
                                    .expect("id always exists")
                                    .clone()
                                }
                                handle_onchange = {
                                    ctx.link().callback(move |(id, txt)| {
                                        Msg::ParamValueChanged(id, txt)
                                    })
                                }
                            />

                            <MySelect
                                id={GEOLOC_SENSOR.id}
                                label={GEOLOC_SENSOR.label}
                                description={GEOLOC_SENSOR.description}
                                select_options={GEOLOC_SENSOR.distinct_vals}
                                vval={
                                    self.vvals.get_by_id(GEOLOC_SENSOR.id)
                                    .expect("id is always valid")
                                    .expect("id always exists")
                                    .clone()
                                }
                                handle_onchange = {
                                    ctx.link().callback(move |(id, txt)| {
                                        Msg::ParamValueChanged(id, txt)
                                    })
                                }
                            />

                            <MySelect
                                id={GEOLOC_METHOD.id}
                                label={GEOLOC_METHOD.label}
                                description={GEOLOC_METHOD.description}
                                select_options={GEOLOC_METHOD.distinct_vals}
                                vval={
                                    self.vvals.get_by_id(GEOLOC_METHOD.id)
                                    .expect("id is always valid")
                                    .expect("id always exists")
                                    .clone()
                                }
                                handle_onchange = {
                                    ctx.link().callback(move |(id, txt)| {
                                        Msg::ParamValueChanged(id, txt)
                                    })
                                }
                            />


                            // <div>
                            //     <MySelect
                            //         id={TRANSMIT_STRAT.id}
                            //         label={TRANSMIT_STRAT.label}
                            //         description={TRANSMIT_STRAT.description}
                            //         select_options={TRANSMIT_STRAT.distinct_vals}
                            //         value={state.transmit_strat.clone()}
                            //         handle_onchange={param_value_changed.clone()}
                            //     />
                            //     <div
                            //         hidden={ state.transmit_strat.clone() != ParamValue::Valid(TransmitStratOption::CUSTOM.val) }
                            //         class={"ml-5 mt-2"}
                            //     >
                            //         <MyTransmitStratCustom
                            //             id={TRANSMIT_STRAT_CUSTOM.id}
                            //             label={TRANSMIT_STRAT_CUSTOM.label}
                            //             description={TRANSMIT_STRAT_CUSTOM.description}
                            //             items={TRANSMIT_STRAT_CUSTOM.bits}
                            //             value={state.transmit_strat_custom.clone()}
                            //             handle_onchange={param_value_changed.clone()}
                            //         />
                            //     </div>
                            // </div>

                            <MyBitmap
                                id={CONFIG_FLAGS.id}
                                label={CONFIG_FLAGS.label}
                                description={CONFIG_FLAGS.description}
                                items={CONFIG_FLAGS.bits}
                                vval={
                                    self.vvals.get_by_id(CONFIG_FLAGS.id)
                                    .expect("id is always valid")
                                    .expect("id always exists")
                                    .clone()
                                }
                                handle_onchange = {
                                    ctx.link().callback(move |(id, txt)| {
                                        Msg::ParamValueChanged(id, txt)
                                    })
                                }
                            />

                        </div>
                    </div>
                </main>
            </>
        )
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ParamValueChanged(id, val) => {
                self.vvals.set_txt_by_id(id, &val).expect("id is always valid");
                true
            },
            // _ => false
        }
    }

}


#[function_component(App)]
pub fn app() -> Html {
    // States variables

    let state = use_state(|| {
        let param_values: ParamValues = Default::default();
        param_values
    });

    // let vvals_state = use_state(|| {
    //     let vvals: PrmVVals = Default::default();
    //     vvals
    // });

    let get_serialport_modal_is_visible = use_state(|| false);
    let save_serialport_modal_is_visible = use_state(|| false);

    let serial_ports = use_state(|| {
        let v: Vec<String> = Vec::new();
        v
    });
    let selected_serial_port = use_state(|| "".to_string());

    let greet_msg = use_state(|| String::new());

    // let force_update = use_force_update();

    // **************************************************************
    // Callbacks

    // param_value_changed: Callback<ValueUpdateData>
    // on_submit
    // on_open
    // on_save_as
    // on_get_config_usb_modal_toggle
    // on_save_config_usb_modal_toggle
    // get_serial_ports_for_get
    // get_serial_ports_for_save
    // onchange_selected_usb_port
    // onclick_get_config_usb
    // onclick_save_config_usb
    // onclick_navbar
    

    // let param_value_changed: Callback<ValueUpdateData> = {
    //     let force_update = force_update.clone();
    //     let state = state.clone();

    //     Callback::from(move |value_update_data: ValueUpdateData| {
    //         if value_update_data.param_id == MODE.id {
    //             state.set(ParamValues {
    //                 mode: value_update_data.new_param_value,
    //                 ..state.deref().clone()
    //             });
    //         } else if value_update_data.param_id == UL_PERIOD.id {
    //             state.set(ParamValues {
    //                 ul_period: value_update_data.new_param_value,
    //                 ..state.deref().clone()
    //             });
    //         } else if value_update_data.param_id == LORA_PERIOD.id {
    //             state.set(ParamValues {
    //                 lora_period: value_update_data.new_param_value,
    //                 ..state.deref().clone()
    //             });
    //         } else if value_update_data.param_id == PERIODIC_POS_PERIOD.id {
    //             state.set(ParamValues {
    //                 periodic_pos_period: value_update_data.new_param_value,
    //                 ..state.deref().clone()
    //             });
    //         } else if value_update_data.param_id == GEOLOC_SENSOR.id {
    //             state.set(ParamValues {
    //                 geoloc_sensor: value_update_data.new_param_value,
    //                 ..state.deref().clone()
    //             });
    //         } else if value_update_data.param_id == GEOLOC_METHOD.id {
    //             state.set(ParamValues {
    //                 geoloc_method: value_update_data.new_param_value,
    //                 ..state.deref().clone()
    //             });
    //         } else if value_update_data.param_id == TRANSMIT_STRAT.id {
    //             state.set(ParamValues {
    //                 transmit_strat: value_update_data.new_param_value,
    //                 ..state.deref().clone()
    //             });
    //             force_update.force_update();
    //         } else if value_update_data.param_id == TRANSMIT_STRAT_CUSTOM.id {
    //             state.set(ParamValues {
    //                 transmit_strat_custom: value_update_data.new_param_value,
    //                 ..state.deref().clone()
    //             });
    //         } else if value_update_data.param_id == CONFIG_FLAGS.id {
    //             state.set(ParamValues {
    //                 config_flags: value_update_data.new_param_value,
    //                 ..state.deref().clone()
    //             });
    //         }
    //     })
    // };

    let on_submit = {
        let greet_msg = greet_msg.clone();
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            let serialized_state = toml::to_string(&*state).unwrap();
            greet_msg.set(serialized_state);
        })
    };

    let on_open = {
        Callback::from(move |_| {
            let args = to_value(&()).unwrap();
            spawn_local(async move {
                invoke("open", args).await; // .as_string().unwrap();
            });
        })
    };

    let on_save_as = {
        let state = state.clone();
        Callback::from(move |_| {
            let param_values: ParamValues = (*state).clone();
            let args = to_value(&SaveArgs { param_values }).unwrap();
            spawn_local(async move {
                invoke("save_as", args).await; //.as_string().unwrap();
            });
        })
    };

    let on_get_config_usb_modal_toggle = {
        let get_serialport_modal_is_visible = get_serialport_modal_is_visible.clone();
        Callback::from(move |_| {
            if *get_serialport_modal_is_visible {
                get_serialport_modal_is_visible.set(false);
            } else {
                get_serialport_modal_is_visible.set(true);
            }
        })
    };

    let on_save_config_usb_modal_toggle = {
        let save_serialport_modal_is_visible = save_serialport_modal_is_visible.clone();
        Callback::from(move |_| {
            if *save_serialport_modal_is_visible {
                save_serialport_modal_is_visible.set(false);
            } else {
                save_serialport_modal_is_visible.set(true);
            }
        })
    };

    let get_serial_ports_for_get = {
        let serial_ports = serial_ports.clone();
        let on_get_config_usb_modal_toggle = on_get_config_usb_modal_toggle.clone();
        Callback::from(move |_: ()| {
            let args = to_value(&()).unwrap();
            let serial_ports = serial_ports.clone();
            let on_get_config_usb_modal_toggle = on_get_config_usb_modal_toggle.clone();
            spawn_local(async move {
                let ports = invoke("get_serial_ports", args).await;
                let ports: Vec<String> = serde_wasm_bindgen::from_value(ports).unwrap();
                log!(format!("KAKUKKK: {:?}", &ports));
                serial_ports.set(ports);
                on_get_config_usb_modal_toggle.emit(());
            });
        })
    };

    let get_serial_ports_for_save = {
        let serial_ports = serial_ports.clone();
        let on_save_config_usb_modal_toggle = on_save_config_usb_modal_toggle.clone();
        Callback::from(move |_: ()| {
            let args = to_value(&()).unwrap();
            let serial_ports = serial_ports.clone();
            let on_save_config_usb_modal_toggle = on_save_config_usb_modal_toggle.clone();
            spawn_local(async move {
                let ports = invoke("get_serial_ports", args).await;
                let ports: Vec<String> = serde_wasm_bindgen::from_value(ports).unwrap();
                log!(format!("KAKUKKK: {:?}", &ports));
                serial_ports.set(ports);
                on_save_config_usb_modal_toggle.emit(());
            });
        })
    };

    let onchange_selected_usb_port = {
        let selected_serial_port = selected_serial_port.clone();
        Callback::from(move |value: String| {
            selected_serial_port.set(value);
        })
    };

    let onclick_get_config_usb = {
        let selected_serial_port = selected_serial_port.clone();
        let state = state.clone();
        let on_get_config_usb_modal_toggle = on_get_config_usb_modal_toggle.clone();
        Callback::from(move |_| {
            log!(&*selected_serial_port);

            if &*selected_serial_port != "" {
                let args = to_value(&GetConfigUsbArgs {
                    port: (&*selected_serial_port).clone(),
                })
                .unwrap();

                let state = state.clone();
                let on_get_config_usb_modal_toggle = on_get_config_usb_modal_toggle.clone();
                spawn_local(async move {
                    let config = invoke("get_config_usb_cmd", args).await;
                    let config: String = serde_wasm_bindgen::from_value(config).unwrap();

                    // log!(&config);

                    let param_values: Result<ParamValues, toml::de::Error> =
                        toml::from_str(&config);

                    match param_values {
                        Ok(param_values) => {
                            log!(format!("{:?}", param_values));
                            state.set(param_values);
                            on_get_config_usb_modal_toggle.emit(());
                        }
                        Err(_err) => {}
                    }
                });
            }
        })
    };

    let onclick_save_config_usb = {
        let state = state.clone();
        let selected_serial_port = selected_serial_port.clone();
        let on_save_config_usb_modal_toggle = on_save_config_usb_modal_toggle.clone();
        Callback::from(move |_| {
            log!(&*selected_serial_port);

            if &*selected_serial_port != "" {
                let param_values: ParamValues = (*state).clone();
                let cli_cmds = param_values.to_cli_cmds();
                let port = (*selected_serial_port).clone();
                let args = to_value(&SaveConfigToUsbArgs { cli_cmds, port }).unwrap();
                let on_save_config_usb_modal_toggle = on_save_config_usb_modal_toggle.clone();
                spawn_local(async move {
                    invoke("save_config_to_usb_cmd", args).await; //.as_string().unwrap();
                    on_save_config_usb_modal_toggle.emit(());
                });

            }
        })
    };

    let onclick_navbar = {
        // let on_get_config_usb_modal_toggle = on_get_config_usb_modal_toggle.clone();
        // let on_save_config_usb_modal_toggle = on_save_config_usb_modal_toggle.clone();
        Callback::from(move |navbar_action| {
            match navbar_action {
                NavbarAction::GetFromFile => {
                    on_open.emit(());
                }
                NavbarAction::SaveToFile => {
                    on_save_as.emit(());
                }
                NavbarAction::GetFromDeviceUSB => {
                    get_serial_ports_for_get.emit(());
                    // on_get_config_usb_modal_toggle.emit(());
                }
                NavbarAction::SaveToDeviceUSB => {
                    get_serial_ports_for_save.emit(());
                    // on_save_config_usb_modal_toggle.emit(());
                }
            }
        })
    };

    #[derive(Deserialize, Debug)]
    struct MenuClickEvent {
        payload: (String, String),
    }

    #[derive(Deserialize, Debug)]
    struct GetSerialPortsEvent {
        payload: Vec<String>,
    }

    {
        let state = state.clone();
        let on_get_config_usb_modal_toggle = on_get_config_usb_modal_toggle.clone();
        // let serial_ports = serial_ports.clone();

        use_effect(move || {
            let menu_click_closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
                let file_open_event: MenuClickEvent = serde_wasm_bindgen::from_value(raw).unwrap();

                match file_open_event.payload.0.as_str() {
                    "FileOpen" => {
                        log!(format!("{}", &file_open_event.payload.0));
                        let param_values: ParamValues =
                            toml::from_str(&file_open_event.payload.1).unwrap();
                        dbg!(&param_values);
                        // state_dispatch.set(param_values);
                        state.set(param_values);
                    }
                    "FileSave" => {
                        log!(format!("{}", &file_open_event.payload.0));

                        let param_values: ParamValues = (*state).clone();
                        let args = to_value(&SaveArgs { param_values }).unwrap();
                        spawn_local(async move {
                            invoke("save_as", args).await; //.as_string().unwrap();
                        });
                    }
                    _ => {}
                }
            });
            let unlisten_menu_click = listen_("MenuClick", &menu_click_closure);

            let get_serial_ports_closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
                let get_serial_ports_event: GetSerialPortsEvent =
                    serde_wasm_bindgen::from_value(raw).unwrap();
                log!(format!("{:?}", &get_serial_ports_event.payload));
                // serial_ports.set(get_serial_ports_event.payload);
                on_get_config_usb_modal_toggle.emit(());
            });
            let unlisten_get_serial_ports = listen_("GetSerialPorts", &get_serial_ports_closure);

            let listeners = (
                unlisten_menu_click,
                unlisten_get_serial_ports,
                menu_click_closure,
                get_serial_ports_closure,
            );
            || {
                let promise_0 = listeners.0.clone();
                let promise_1 = listeners.1.clone();
                spawn_local(async move {
                    let unlisten_0: Function = wasm_bindgen_futures::JsFuture::from(promise_0)
                        .await
                        .unwrap()
                        .into();

                    let unlisten_1: Function = wasm_bindgen_futures::JsFuture::from(promise_1)
                        .await
                        .unwrap()
                        .into();

                    unlisten_0.call0(&JsValue::undefined()).unwrap();
                    unlisten_1.call0(&JsValue::undefined()).unwrap();
                });

                // spawn_local(async move {
                //     let unlisten_1: Function = wasm_bindgen_futures::JsFuture::from(promise_1)
                //         .await
                //         .unwrap()
                //         .into();
                //     unlisten_1.call0(&JsValue::undefined()).unwrap();
                // });

                drop(listeners);
            }
        });
    }


    html! {

        <>

            <Navbar onclick={onclick_navbar} />

            <main class="pt-14">





                <Modal
                    title="Get config from device"
                    is_visible={&*get_serialport_modal_is_visible}
                    onclose={on_get_config_usb_modal_toggle.clone()}
                >

                    <SelectUsbPort
                        id = {"selected-usb-port"}
                        label = {"Select the USB port connecting the device"}
                        description = {"Select the USB port connecting the device"}
                        select_options = {(*serial_ports).clone()}
                        value = {(*selected_serial_port).clone()}
                        handle_onchange = { onchange_selected_usb_port.clone() }
                    />

                   <button
                        class="mt-5 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        onclick={onclick_get_config_usb.clone()}
                    >
                        {"Get Config from Device"}
                    </button>

                </Modal>


                <Modal
                    title="Save config to device"
                    is_visible={&*save_serialport_modal_is_visible}
                    onclose={on_save_config_usb_modal_toggle.clone()}
                >

                    <SelectUsbPort
                        id = {"selected-usb-port"}
                        label = {"Select the USB port connecting the device"}
                        description = {"Select the USB port connecting the device"}
                        select_options = {(*serial_ports).clone()}
                        value = {(*selected_serial_port).clone()}
                        handle_onchange = { onchange_selected_usb_port.clone() }
                    />

                   <button
                        class="mt-5 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        onclick={onclick_save_config_usb.clone()}
                    >
                        {"Save Config to Device"}
                    </button>

                </Modal>




                <div class="m-7">
                    <div class="grid gap-5 mb-5 md:grid-cols-2">

                        // <MySelect
                        //     id={MODE.id}
                        //     label={MODE.label}
                        //     description={MODE.description}
                        //     select_options={MODE.distinct_vals}
                        //     value={state.mode.clone()}
                        //     handle_onchange={param_value_changed.clone()}
                        // />

                        // <MyInput
                        //     id={UL_PERIOD.id}
                        //     label={UL_PERIOD.label}
                        //     description={UL_PERIOD.description}
                        //     value={state.ul_period.clone()}
                        //     valid_range={UL_PERIOD.valid_range}
                        //     handle_onchange={param_value_changed.clone()}
                        // />

                        // <MyHexInput
                        //     id={LORA_PERIOD.id}
                        //     label={LORA_PERIOD.label}
                        //     description={LORA_PERIOD.description}
                        //     value={state.lora_period.clone()}
                        //     valid_range={LORA_PERIOD.range}
                        //     handle_onchange={param_value_changed.clone()}
                        // />

                        // <MyOptionalInput
                        //     id={PERIODIC_POS_PERIOD.id}
                        //     label={PERIODIC_POS_PERIOD.label}
                        //     description={PERIODIC_POS_PERIOD.description}
                        //     value={state.periodic_pos_period.clone()}
                        //     disabled_value={PERIODIC_POS_PERIOD.disabled_val}
                        //     default_value={PERIODIC_POS_PERIOD.default_val}
                        //     valid_range={PERIODIC_POS_PERIOD.range}
                        //     handle_onchange={param_value_changed.clone()}
                        // />

                        // <MySelect
                        //     id={GEOLOC_SENSOR.id}
                        //     label={GEOLOC_SENSOR.label}
                        //     description={GEOLOC_SENSOR.description}
                        //     select_options={GEOLOC_SENSOR.distinct_vals}
                        //     value={state.geoloc_sensor.clone()}
                        //     handle_onchange={param_value_changed.clone()}
                        // />

                        // <MySelect
                        //     id={GEOLOC_METHOD.id}
                        //     label={GEOLOC_METHOD.label}
                        //     description={GEOLOC_METHOD.description}
                        //     select_options={GEOLOC_METHOD.distinct_vals}
                        //     value={state.geoloc_method.clone()}
                        //     handle_onchange={param_value_changed.clone()}
                        // />

                        // <div>
                        //     <MySelect
                        //         id={TRANSMIT_STRAT.id}
                        //         label={TRANSMIT_STRAT.label}
                        //         description={TRANSMIT_STRAT.description}
                        //         select_options={TRANSMIT_STRAT.distinct_vals}
                        //         value={state.transmit_strat.clone()}
                        //         handle_onchange={param_value_changed.clone()}
                        //     />
                        //     <div
                        //         hidden={ state.transmit_strat.clone() != ParamValue::Valid(TransmitStratOption::CUSTOM.val) }
                        //         class={"ml-5 mt-2"}
                        //     >
                        //         <MyTransmitStratCustom
                        //             id={TRANSMIT_STRAT_CUSTOM.id}
                        //             label={TRANSMIT_STRAT_CUSTOM.label}
                        //             description={TRANSMIT_STRAT_CUSTOM.description}
                        //             items={TRANSMIT_STRAT_CUSTOM.bits}
                        //             value={state.transmit_strat_custom.clone()}
                        //             handle_onchange={param_value_changed.clone()}
                        //         />
                        //     </div>
                        // </div>

                        // <MyBitmap
                        //     id={CONFIG_FLAGS.id}
                        //     label={CONFIG_FLAGS.label}
                        //     description={CONFIG_FLAGS.description}
                        //     items={CONFIG_FLAGS.bits}
                        //     value={state.config_flags.clone()}
                        //     handle_onchange={param_value_changed.clone()}
                        // />

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

                    </div>
                </div>

                // <pre>{ &*state.to_string() }</pre>
                <pre>{ &*greet_msg }</pre>

            </main>

        </>

    }
}
