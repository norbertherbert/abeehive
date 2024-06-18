use std::cell::RefCell;
use std::rc::Rc;

use abeehive::prm::val::PrmVVals;
// use std::fmt;
// use yewdux::prelude::*;
use gloo::console::log;
use js_sys::{
    // Function, 
    Function, Promise
};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
// use std::ops::Deref;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use abeehive::components::RadixDisp;
use abeehive::components::my_input::MyInput;
use abeehive::components::my_optional_input::MyOptionalInput;
use abeehive::components::my_select::MySelect;
use abeehive::components::my_bitmap::MyBitmap;
use abeehive::components::myc_transmit_strat_custom::MycTransmitStratCustom;
use abeehive::components::myc_motion_sensitivity::MycMotionSensitivity;
use abeehive::components::myc_button_mapping::MycButtonMapping;
use abeehive::components::myc_battery_capacity::MycBatteryCapacity;

use abeehive::components::modal::Modal;
use abeehive::components::navbar::{Navbar, NavbarAction};
use abeehive::components::select_usb_port::SelectUsbPort;

use abeehive::prm::{ dat::*, val::PrmVVal, typ::PrmVal };

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window.__TAURI__.event"], js_name = "listen")]
    fn listen_(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> Promise;

}

#[derive(Serialize, Deserialize)]
struct SaveArgs {
    cfg_string: String,
}

#[derive(Serialize, Deserialize)]
struct GetConfigUsbArgs {
    port: String,
}

#[derive(Serialize, Deserialize)]
struct SaveConfigToUsbArgs {
    cli_cmds: Vec<(u8, PrmVal)>,
    port: String,
}

#[derive(Deserialize, Debug)]
struct MenuClickEvent {
    payload: (String, String),
}


#[derive(Debug, PartialEq)]
pub enum Msg {
    ParamValueChanged(u8, String),
    Submit,
    Open,
    SaveAs,
    GetConfigUsbModalToggle,
    SaveConfigUsbModalToggle,
    GetSerialPortsForGetCfg,
    UpdateSerialPortsForGetCfg(Vec<String>),
    GetSerialPortsForSaveCfg,
    UpdateSerialPortsForSaveCfg(Vec<String>),
    SelectedUsbPortChanged(String),
    GetConfigUsb,
    SaveConfigUsb,
    UpdateVVals(PrmVVals),
    Navbar(NavbarAction),
}

#[derive(Default, Debug)]
pub struct BeeQueenApp {
    vvals: Rc<RefCell<PrmVVals>>,
    get_serialport_modal_is_visible: bool,
    save_serialport_modal_is_visible: bool,
    serial_ports: Vec<String>,
    selected_serial_port: String,
    greet_msg: String,
    menu_click_listener: Option<(Promise, Closure<dyn FnMut(JsValue)>)>,
}

impl Component for BeeQueenApp {
    type Message = Msg;
    type Properties = ();
  
    fn create(_ctx: &Context<Self>) -> Self {

        let beequeen_app = BeeQueenApp::default();
        
        // // an invalid values to test how the GUI acts in case of errors
        // beequeen_app.vvals.borrow_mut().set_val_by_id(MODE.id, 10).unwrap();
        // beequeen_app.vvals.borrow_mut().set_val_by_id(CONFIG_FLAGS.id, (0b_00000000_11110111_11111111_11111111 as u32) as i32).unwrap();
        beequeen_app.vvals.borrow_mut().set_val_by_id(MOTION_SENSITIVITY.id, 0xff0201).unwrap();
        beequeen_app.vvals.borrow_mut().set_val_by_id(BUTTON_MAPPING.id, 0x44444).unwrap();
        beequeen_app.vvals.borrow_mut().set_val_by_id(BATTERY_CAPACITY.id, -100).unwrap();

        beequeen_app

    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ParamValueChanged(id, val) => {
                self.vvals.borrow_mut().set_txt_by_id(id, &val).expect("id is always valid");
                true
            },
            Msg::Submit => { 
                let serialized_state = self.vvals.borrow_mut().to_cfg_string();
                self.greet_msg = serialized_state;
                true
            },
            Msg::Open => {
                let args = to_value(&()).unwrap();
                spawn_local(async move {
                    invoke("open", args).await; // .as_string().unwrap();
                });
                true
            },
            Msg::SaveAs => {

                let cfg_string = self.vvals.borrow_mut().to_cfg_string();
                let args = to_value(&SaveArgs { cfg_string }).unwrap();
                spawn_local(async move {
                    invoke("save_as", args).await; //.as_string().unwrap();
                });

                true
            },
            Msg::GetConfigUsbModalToggle => {
                self.get_serialport_modal_is_visible = !self.get_serialport_modal_is_visible;
                true
            },
            Msg::SaveConfigUsbModalToggle => {
                self.save_serialport_modal_is_visible = !self.save_serialport_modal_is_visible;
                true
            },
            
            
            Msg::GetSerialPortsForGetCfg => {

                let args = to_value(&()).unwrap();

                let send_ports = ctx.link().callback(move |ports: Vec<String>| {
                    Msg::UpdateSerialPortsForGetCfg(ports)
                });
             
                spawn_local(async move {
                    let ports = invoke("get_serial_ports", args).await;
                    let ports: Vec<String> = serde_wasm_bindgen::from_value(ports).unwrap();
                    log!(format!("KAKUKKK: {:?}", &ports));
                    send_ports.emit(ports);
                });

                true
            },
            Msg::UpdateSerialPortsForGetCfg(ports) => {
                self.serial_ports = ports;
                self.get_serialport_modal_is_visible = true;
                true
            },

            
            Msg::GetSerialPortsForSaveCfg => {

                let args = to_value(&()).unwrap();

                let send_ports = ctx.link().callback(move |ports: Vec<String>| {
                    Msg::UpdateSerialPortsForSaveCfg(ports)
                });
             
                spawn_local(async move {
                    let ports = invoke("get_serial_ports", args).await;
                    let ports: Vec<String> = serde_wasm_bindgen::from_value(ports).unwrap();
                    log!(format!("KAKUKKK: {:?}", &ports));
                    send_ports.emit(ports);
                });

                true
            },
            Msg::UpdateSerialPortsForSaveCfg(ports) => {
                self.serial_ports = ports;
                self.save_serialport_modal_is_visible = true;
                true
            },


            Msg::SelectedUsbPortChanged(port) => {
                self.selected_serial_port = port;
                true
            },


            Msg::GetConfigUsb => {

                log!(&self.selected_serial_port);

                if !self.selected_serial_port.is_empty() {
                    let args = to_value(&GetConfigUsbArgs {
                        port: self.selected_serial_port.clone(),
                    })
                    .unwrap();

                    let update_vvals = ctx.link().callback(move |vvals: PrmVVals| {
                        Msg::UpdateVVals(vvals)
                    });

                    // let cloned_vvals = self.vvals.clone();

                    spawn_local(async move {
                        let cfg = invoke("get_config_usb_cmd", args).await;
                        let cfg_vec: Vec<(u8, PrmVal)> = serde_wasm_bindgen::from_value(cfg).unwrap();
    
                        // log!(&cfg_string);

                        let vvals = PrmVVals::from_cfg_vec(&cfg_vec);
                        match vvals {
                            Ok(vvals) => {
                                // log!(format!("{:?}", vvals));
                                update_vvals.emit(vvals);
                            }
                            Err(_err) => {} // TODO!
                        }

                    });
                }

                true
            },

            Msg::UpdateVVals(vvals) => {
                // log!(format!("{:?}", vvals));
                self.vvals.replace(vvals);
                self.get_serialport_modal_is_visible = false;
                true
            }



            Msg::SaveConfigUsb => {

                log!(&self.selected_serial_port);

                if !self.selected_serial_port.is_empty() {

                    let cli_cmds = self.vvals.borrow().to_cfg_vec().clone();
                    let port = self.selected_serial_port.clone();
                    let args = to_value(&SaveConfigToUsbArgs{ cli_cmds, port }).unwrap();

                    let save_config_usb_modal_toggle = ctx.link().callback(move |()| {
                        Msg::SaveConfigUsbModalToggle
                    });

                    spawn_local(async move {
                        invoke("save_config_to_usb_cmd", args).await; //.as_string().unwrap();
                        save_config_usb_modal_toggle.emit(())
                    });
    
                }

                true
            },



            Msg::Navbar(navbar_action) => {
                match navbar_action {
                    NavbarAction::GetFromFile => {
                        ctx.link().callback(move |_: ()| {
                            Msg::Open
                        }).emit(());
                    },
                    NavbarAction::SaveToFile => {
                        ctx.link().callback(move |_: ()| {
                            Msg::SaveAs
                        }).emit(());
                    },
                    NavbarAction::GetFromDeviceUSB => {
                        ctx.link().callback(move |_: ()| {
                            Msg::GetSerialPortsForGetCfg
                        }).emit(());
                    },
                    NavbarAction::SaveToDeviceUSB => {
                        ctx.link().callback(move |_: ()| {
                            Msg::GetSerialPortsForSaveCfg
                        }).emit(());
                    },
                }
                true
            }

            // Msg::GetCfgString => {
            //     self.vvals.to_cfg_string();
            //     true
            // }

        }
    }


    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {

        let update_vvals = ctx.link().callback(move |vvals: PrmVVals| {
            Msg::UpdateVVals(vvals)
        });
 
        let menu_click_closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {

            let file_open_event: MenuClickEvent = serde_wasm_bindgen::from_value(raw).unwrap();

            match file_open_event.payload.0.as_str() {
                "FileOpen" => {

                    log!(format!("{}", &file_open_event.payload.0));
                    log!(format!("{}", &file_open_event.payload.1));

                    let vvals = PrmVVals::from_cfg_str(&file_open_event.payload.1)
                    .unwrap();

                    log!(format!("{:?}", &vvals));

                    update_vvals.emit(vvals);

                }
                _ => {}
            }

        });

        let unlisten_menu_click = listen_("MenuClick", &menu_click_closure);

        self.menu_click_listener = Some((
            unlisten_menu_click,
            menu_click_closure,
        ));

    }

    fn destroy(&mut self, _ctx: &Context<Self>) {

        if let Some(ref menu_click_listener) = self.menu_click_listener {
        
            let unlisten_menu_click_promise = menu_click_listener.0.clone();
        
            spawn_local(async move {

                let unlisten_menu_click: Function = wasm_bindgen_futures::JsFuture::from(unlisten_menu_click_promise)
                .await
                .unwrap()
                .into();

                unlisten_menu_click.call0(&JsValue::undefined()).unwrap();

            });

            self.menu_click_listener = None;

        }

    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <>

                <Navbar 
                    onclick = { ctx.link().callback(move |navbar_action: NavbarAction| {
                        Msg::Navbar(navbar_action)
                    }) } 
                />

                <main class="pt-14">

                    <Modal
                        title = "Get config from device"
                        is_visible = { self.get_serialport_modal_is_visible }
                        onclose = { ctx.link().callback(move |_| {
                            Msg::GetConfigUsbModalToggle
                        }) } 
                    >

                        <SelectUsbPort
                            id = { "selected-usb-port" }
                            label = { "Select the USB port connecting the device" }
                            description = { "Select the USB port connecting the device" }
                            select_options = { self.serial_ports.clone() }
                            value = { self.selected_serial_port.clone() }
                            handle_onchange = { ctx.link().callback(move |port: String| {
                                Msg::SelectedUsbPortChanged(port)
                            }) } 
                        />

                        <button
                            class="mt-5 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                            onclick = { ctx.link().callback(move |_| {
                                Msg::GetConfigUsb
                            }) } 
                        >
                            { "Get Config from Device" }
                        </button>

                    </Modal>

                    <Modal
                        title = "Save config to device"
                        is_visible = { self.save_serialport_modal_is_visible }
                        onclose = { ctx.link().callback(move |_| {
                            Msg::SaveConfigUsbModalToggle
                        }) } 
                    >

                        <SelectUsbPort
                            id = { "selected-usb-port" }
                            label = { "Select the USB port connecting the device" }
                            description = { "Select the USB port connecting the device" }
                            select_options = { self.serial_ports.clone() }
                            value = { self.selected_serial_port.clone() }
                            handle_onchange = { ctx.link().callback(move |port: String| {
                                Msg::SelectedUsbPortChanged(port)
                            }) }
                        />

                        <button
                            class="mt-5 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                            onclick = { ctx.link().callback(move |_| {
                                Msg::SaveConfigUsb
                            }) } 
                        >
                            { "Save Config to Device" }
                        </button>

                    </Modal>
              
                    // PARAMETER COOMPONENTS:

                    // <div class="m-7 flex flex-col flex-wrap gap-5">
                    <div class="m-7 grid gap-5 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6">

                        <MySelect
                            id={MODE.id}
                            label={MODE.label}
                            description={MODE.description}
                            select_options={MODE.distinct_vals}

                            vval={
                                self.vvals.borrow().get_by_id(MODE.id)
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
                                self.vvals.borrow().get_by_id(UL_PERIOD.id)
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
                            radix_disp = { RadixDisp::Dec }
                            vval={
                                self.vvals.borrow().get_by_id(LORA_PERIOD.id)
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
                                self.vvals.borrow().get_by_id(PERIODIC_POS_PERIOD.id)
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
                                self.vvals.borrow().get_by_id(GEOLOC_SENSOR.id)
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
                                self.vvals.borrow().get_by_id(GEOLOC_METHOD.id)
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
                            id={TRANSMIT_STRAT.id}
                            label={TRANSMIT_STRAT.label}
                            description={TRANSMIT_STRAT.description}
                            select_options={TRANSMIT_STRAT.distinct_vals}

                            vval={
                                self.vvals.borrow().get_by_id(TRANSMIT_STRAT.id)
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


                        <div
                            hidden = { 
                                self.vvals.borrow().get_by_id(TRANSMIT_STRAT.id)
                                .expect("id is always valid")
                                .expect("id always exists")
                                .clone()
                                !=
                                PrmVVal::Valid(TransmitStratOption::CUSTOM.val) 
                            }
                            class = "col-span-1 row-span-3"
                        >
                            <MycTransmitStratCustom
                                id={TRANSMIT_STRAT_CUSTOM.id}
                                label={TRANSMIT_STRAT_CUSTOM.label}
                                description={TRANSMIT_STRAT_CUSTOM.description}
                                items={TRANSMIT_STRAT_CUSTOM.bits}
                                vval={
                                    self.vvals.borrow().get_by_id(TRANSMIT_STRAT_CUSTOM.id)
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


                        <MyBitmap
                            id={CONFIG_FLAGS.id}
                            label={CONFIG_FLAGS.label}
                            description={CONFIG_FLAGS.description}
                            items={CONFIG_FLAGS.bits}
                            vval={
                                self.vvals.borrow().get_by_id(CONFIG_FLAGS.id)
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


                        <div class = "col-span-1 row-span-2">
                            <MycMotionSensitivity
                                id = {MOTION_SENSITIVITY.id}
                                label = {MOTION_SENSITIVITY.label}
                                description = {MOTION_SENSITIVITY.description}
                                default_val = {MOTION_SENSITIVITY.default_val}
                                range_sensitivity = {MOTION_SENSITIVITY.range_sensitivity}
                                distinct_vals_odr = {MOTION_SENSITIVITY.distinct_vals_odr}
                                distinct_vals_fullscale = {MOTION_SENSITIVITY.distinct_vals_fullscale}

                                vval={
                                    self.vvals.borrow().get_by_id(MOTION_SENSITIVITY.id)
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



                        <div class = "col-span-1 row-span-4">
                            <MycButtonMapping
                                id = {BUTTON_MAPPING.id}
                                label = {BUTTON_MAPPING.label}
                                description = {BUTTON_MAPPING.description}
                                default_val = {BUTTON_MAPPING.default_val}
                                long_press_duration_range = {BUTTON_MAPPING.long_press_duration_range}
                                action_distinct_vals = {BUTTON_MAPPING.action_distinct_vals}

                                vval={
                                    self.vvals.borrow().get_by_id(BUTTON_MAPPING.id)
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

                        <div class = "col-span-1 row-span-2">
                            <MycBatteryCapacity
                                id = {BATTERY_CAPACITY.id}
                                label = {BATTERY_CAPACITY.label}
                                description = {BATTERY_CAPACITY.description}
                                default_val = {BATTERY_CAPACITY.default_val}
                                distinct_vals = {BATTERY_CAPACITY.distinct_vals}

                                vval={
                                    self.vvals.borrow().get_by_id(BATTERY_CAPACITY.id)
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

                    <div>

                        <button
                            id = "btn-submit"
                            data-tooltip-target = "submit-tooltip"
                            class = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                            onclick = { ctx.link().callback(move |_| {
                                Msg::Submit
                            }) } 
                        >
                            { "Submit" }
                        </button>
                        <div
                            id = "submit-tooltip"
                            role = "tooltip"
                            class = "absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-white transition-opacity duration-300 bg-gray-900 rounded-lg shadow-sm opacity-0 tooltip dark:bg-gray-700"
                        >
                            { "Click to submit the form!" }
                            <div class="tooltip-arrow" data-popper-arrow="true"></div>
                        </div>

                    </div>

                    <pre>{ &self.greet_msg }</pre>

                </main>
            </>
        )
    }

}

