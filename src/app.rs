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

    #[wasm_bindgen(js_name = "initFlowbite")]
    fn init_flowbite();

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window.__TAURI__.event"], js_name = "listen")]
    fn listen_(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> Promise;

}

// #[wasm_bindgen(module = "/assets/flowbite.js")]
// extern "C" {
//     #[wasm_bindgen(js_name = "initFlowbite")]
//     fn init_flowbite();
// }

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
struct FileOpenEvent {
    payload: (String, String),
}

#[derive(Deserialize, Debug)]
struct SaveAsEvent {
    payload: String
}

#[derive(Debug, PartialEq)]
pub enum Msg {
    ParamValueChanged(u8, String),
    ShowCfg,
    ShowLWDL,
    New,
    Close,
    Open,
    OpenWith,
    SaveAs,
    ExportAsLWDL,
    GetConfigUsbModalToggle,
    SaveConfigUsbModalToggle,
    GetSerialPortsForGetCfg,
    UpdateSerialPortsForGetCfg(Vec<String>),
    GetSerialPortsForSaveCfg,
    UpdateSerialPortsForSaveCfg(Vec<String>),
    SelectedUsbPortChanged(String),
    GetConfigUsb,
    SaveConfigUsb,
    UpdateVVals((String, PrmVVals)),
    UpdateSourceName(String),
    Navbar(NavbarAction),
}

#[derive(Default, Debug)]
pub struct BeeQueenApp {
    source_name: String,
    vvals: Rc<RefCell<PrmVVals>>,
    get_serialport_modal_is_visible: bool,
    save_serialport_modal_is_visible: bool,
    serial_ports: Vec<String>,
    selected_serial_port: String,
    greet_msg: String,
    file_open_listener: Option<(Promise, Closure<dyn FnMut(JsValue)>)>,
    save_as_listener: Option<(Promise, Closure<dyn FnMut(JsValue)>)>,
}


impl Component for BeeQueenApp {
    type Message = Msg;
    type Properties = ();
  
    fn create(_ctx: &Context<Self>) -> Self {

        // let beequeen_app = BeeQueenApp::default();

        let beequeen_app = BeeQueenApp {
            source_name: String::new(),
            vvals: Rc::new(RefCell::new(PrmVVals::new())),
            get_serialport_modal_is_visible: false,
            save_serialport_modal_is_visible: false,
            serial_ports: Vec::new(),
            selected_serial_port: String::new(),
            greet_msg: String::new(),
            file_open_listener: None,
            save_as_listener: None,
        };
        
        // // an invalid values to test how the GUI acts in case of errors
        // beequeen_app.vvals.borrow_mut().set_val_by_id(MODE.id, 10).unwrap();
        // beequeen_app.vvals.borrow_mut().set_val_by_id(CONFIG_FLAGS.id, (0b_00000000_11110111_11111111_11111111 as u32) as i32).unwrap();
        // beequeen_app.vvals.borrow_mut().set_val_by_id(MOTION_SENSITIVITY.id, 0xff0201).unwrap();
        // beequeen_app.vvals.borrow_mut().set_val_by_id(BUTTON_MAPPING.id, 0x44444).unwrap();
        // beequeen_app.vvals.borrow_mut().set_val_by_id(BATTERY_CAPACITY.id, -100).unwrap();

        beequeen_app

    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ParamValueChanged(id, val) => {
                self.vvals.borrow_mut().set_txt_by_id(id, &val).expect("id is always valid");
                self.greet_msg = String::new();
                true
            },

            Msg::ShowCfg => { 
                let serialized_state = self.vvals.borrow_mut().to_cfg_string();
                self.greet_msg = serialized_state;
                true
            },
            Msg::ShowLWDL => { 
                let serialized_state = self.vvals.borrow_mut().to_lwdl_string();
                self.greet_msg = serialized_state;
                true
            },

            Msg::New => {
                let vvals = PrmVVals::default();
                self.source_name = "New Configuration".to_string();
                self.vvals.replace(vvals);
                true
            },
            Msg::Close => {
                let vvals = PrmVVals::new();
                self.vvals.replace(vvals);
                self.source_name = String::new();
                true
            },

            Msg::Open => {
                let args = to_value(&()).unwrap();
                spawn_local(async move {
                    invoke("open", args).await; // .as_string().unwrap();
                });
                true
            },
            Msg::OpenWith => {
                let args = to_value(&()).unwrap();
                spawn_local(async move {
                    invoke("open_with", args).await; // .as_string().unwrap();
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
            Msg::ExportAsLWDL => {

                let lwdl_string = self.vvals.borrow_mut().to_lwdl_string();
                let args = to_value(&SaveArgs { cfg_string: lwdl_string }).unwrap();
                spawn_local(async move {
                    invoke("save_as", args).await; //.as_string().unwrap();
                });

                true
            }
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

                    let selected_serial_port = self.selected_serial_port.clone();
                    let update_vvals = ctx.link().callback(move |vvals: PrmVVals| {
                        Msg::UpdateVVals(( format!("USB port: {}", selected_serial_port), vvals ))
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

            Msg::UpdateVVals(( source_name, vvals )) => {
                // log!(format!("{:?}", vvals));
                self.source_name = source_name;
                self.vvals.replace(vvals);
                self.get_serialport_modal_is_visible = false;
                true
            }

            Msg::UpdateSourceName( source_name) => {
                self.source_name = source_name;
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

                    NavbarAction::New => {
                        ctx.link().callback(move |_: ()| {
                            Msg::New
                        }).emit(());
                    },
                    NavbarAction::Close => {
                        ctx.link().callback(move |_: ()| {
                            Msg::Close
                        }).emit(());
                    },

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
                    
                    NavbarAction::ExportToLWDLFile => {
                        ctx.link().callback(move |_: ()| {
                            Msg::ExportAsLWDL
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


    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {

        if first_render { 

            let open_with = ctx.link().callback(move |_| {
                Msg::OpenWith
            });
            open_with.emit(());

        }

        // ************************************************************************
        // *** Initiating the Flowbite framework as an external Javascript Call 
        // ************************************************************************

        init_flowbite();


        // *************************************
        // *** Handling the "FileOpen" Event
        // *************************************

        let update_on_file_open = ctx.link().callback(move | args: (String, PrmVVals)| {
            Msg::UpdateVVals(args)
        });
 
        let file_open_closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {

            let file_open_event: FileOpenEvent = serde_wasm_bindgen::from_value(raw).unwrap();

            log!(format!("{}", &file_open_event.payload.0));
            // log!(format!("{}", &file_open_event.payload.1));

            let vvals = PrmVVals::from_cfg_str(&file_open_event.payload.1)
            .unwrap();

            // log!(format!("{:?}", &vvals));

            update_on_file_open.emit((file_open_event.payload.0, vvals));

        });

        let unlisten_file_open = listen_("FileOpen", &file_open_closure);

        self.file_open_listener = Some((
            unlisten_file_open,
            file_open_closure,
        ));


        // *************************************
        // *** Handling the "SaveAs" Event
        // *************************************


        let update_on_save_as = ctx.link().callback(move | file_name: String | {
            Msg::UpdateSourceName(file_name)
        });
 
        let save_as_closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
            let save_as_event: SaveAsEvent = serde_wasm_bindgen::from_value(raw).unwrap();
            log!(format!("{}", &save_as_event.payload));
            update_on_save_as.emit(save_as_event.payload);
        });

        let unlisten_save_as = listen_("SaveAs", &save_as_closure);

        self.save_as_listener = Some((
            unlisten_save_as,
            save_as_closure,
        ));
        
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {

        if let Some(ref file_open_listener) = self.file_open_listener {
        
            let unlisten_file_open_promise = file_open_listener.0.clone();
        
            spawn_local(async move {

                let unlisten_file_open: Function = wasm_bindgen_futures::JsFuture::from(unlisten_file_open_promise)
                .await
                .unwrap()
                .into();

                unlisten_file_open.call0(&JsValue::undefined()).unwrap();

            });

            self.file_open_listener = None;

        }

        if let Some(ref save_as_listener) = self.save_as_listener {
        
            let unlisten_save_as_promise = save_as_listener.0.clone();
        
            spawn_local(async move {

                let unlisten_save_as: Function = wasm_bindgen_futures::JsFuture::from(unlisten_save_as_promise)
                .await
                .unwrap()
                .into();

                unlisten_save_as.call0(&JsValue::undefined()).unwrap();

            });

            self.save_as_listener = None;

        }

    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let handle_onchange = {
            ctx.link().callback(move |(id, txt)| {
                Msg::ParamValueChanged(id, txt)
            })
        };

        let mut is_standby = true;
        let mut is_gps_used = false; 
        let mut _is_lpgps_used = false;
        let mut is_ble_position_used = false;
        let mut _is_wifi_used = false;
        let mut is_accelerometer_used = false;

        if !self.vvals.borrow().is_empty() {


            (is_accelerometer_used, is_standby) = 
                match self.vvals.borrow().get_by_id(MODE.id)
                    .expect("id is always valid")
                    .expect("id always exists")
                {
                    PrmVVal::Valid(v) => {
                        match v {
                            v if *v==ModeOption::STANDBY.val => (false, true),
                            v if *v==ModeOption::MOTION_TRACKING.val => (true, false),
                            v if *v==ModeOption::MOTION_START_END_TRACKING.val => (true, false),
                            v if *v==ModeOption::ACTIVITY_TRACKING.val => (true, false),
                            _ => (false, false)
                        }
                    },
                    _ => (false, false)
                };

            is_accelerometer_used |= 
                match self.vvals.borrow().get_by_id(SHOCK_DETECTION.id)
                    .expect("id is always valid")
                    .expect("id always exists")
                {
                    PrmVVal::Valid(v) => {
                        match v {
                            v if *v==SHOCK_DETECTION.disabled_val => false,
                            _ => true,
                        }
                    },
                    _ => false
                };

            is_accelerometer_used |= 
                match self.vvals.borrow().get_by_id(CONFIG_FLAGS.id)
                    .expect("id is always valid")
                    .expect("id always exists")
                {
                    PrmVVal::Valid(bitmap) => {
                        let bitmask: i32 = 
                            (1<<ConfigFlagsBit::MOTION_START_EVENT_ENABLED.bit) |
                            (1<<ConfigFlagsBit::MOTION_END_EVENT_ENABLED.bit);
                        (bitmap & bitmask) > 0
                    },
                    _ => false
                };

            if !is_standby {
                (is_gps_used, _is_lpgps_used, is_ble_position_used, _is_wifi_used) = 
                    match self.vvals.borrow().get_by_id(GEOLOC_SENSOR.id)
                        .expect("id is always valid")
                        .expect("id always exists")
                    {
                        PrmVVal::Valid(v) => {
                            
                            let is_gps_used = match v {
                                v if *v==GeolocSensorOption::GPS.val => true,
                                v if *v==GeolocSensorOption::BLE_GPS.val => true,
                                v if *v==GeolocSensorOption::WIFI_GPS.val => true,
                                v if *v==GeolocSensorOption::WIFI_LPGPS_WIFI_GPS.val => true,
                                _ => false,
                            };

                            let is_lpgps_used = match v {
                                v if *v==GeolocSensorOption::LPGPS.val => true,
                                v if *v==GeolocSensorOption::BLE_LPGPS.val => true,
                                v if *v==GeolocSensorOption::WIFI_LPGPS.val => true,
                                v if *v==GeolocSensorOption::WIFI_LPGPS_WIFI_GPS.val => true,
                                _ => false,
                            };

                            let is_ble_positioning_used = match v {
                                v if *v==GeolocSensorOption::BLE.val => true,
                                v if *v==GeolocSensorOption::BLE_GPS.val => true,
                                v if *v==GeolocSensorOption::BLE_LPGPS.val => true,
                                _ => false,
                            };

                            let is_wifi_used = match v {
                                v if *v==GeolocSensorOption::WIFI.val => true,
                                v if *v==GeolocSensorOption::WIFI_GPS.val => true,
                                v if *v==GeolocSensorOption::WIFI_LPGPS.val => true,
                                v if *v==GeolocSensorOption::WIFI_LPGPS_WIFI_GPS.val => true,
                                _ => false,
                            };

                            (is_gps_used, is_lpgps_used, is_ble_positioning_used, is_wifi_used)

                        },
                        _ => (false, false, false, false) 
                    };
            }

            let (is_gps_used_1, is_lpgps_used_1, is_ble_position_used_1, is_wifi_used_1) = 
                match self.vvals.borrow().get_by_id(GEOLOC_METHOD.id)
                    .expect("id is always valid")
                    .expect("id always exists")
                {
                    PrmVVal::Valid(v) => {
                        
                        let is_gps_used = match v {
                            v if *v==GeolocMethodOption::GPS.val => true,
                            v if *v==GeolocMethodOption::BLE_GPS.val => true,
                            v if *v==GeolocMethodOption::WIFI_GPS.val => true,
                            _ => false,
                        };

                        let is_lpgps_used = match v {
                            v if *v==GeolocMethodOption::LPGPS.val => true,
                            v if *v==GeolocMethodOption::BLE_LPGPS.val => true,
                            v if *v==GeolocMethodOption::WIFI_LPGPS.val => true,
                            _ => false,
                        };

                        let is_ble_position_used = match v {
                            v if *v==GeolocMethodOption::BLE.val => true,
                            v if *v==GeolocMethodOption::BLE_GPS.val => true,
                            v if *v==GeolocMethodOption::BLE_LPGPS.val => true,
                            _ => false,
                        };

                        let is_wifi_used = match v {
                            v if *v==GeolocMethodOption::WIFI.val => true,
                            v if *v==GeolocMethodOption::WIFI_GPS.val => true,
                            v if *v==GeolocMethodOption::WIFI_LPGPS.val => true,
                            _ => false,
                        };

                        (is_gps_used, is_lpgps_used, is_ble_position_used, is_wifi_used)

                    },
                    _ => (false, false, false, false) 
                };

            is_gps_used |= is_gps_used_1;
            _is_lpgps_used |= is_lpgps_used_1;
            is_ble_position_used |= is_ble_position_used_1;
            _is_wifi_used |= is_wifi_used_1;


        }

        html!(
            <>

                <Navbar 
                    source_name = { self.source_name.clone() }
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

                    





                    // -- PARAMETER COOMPONENTS:

                    {
                        if self.vvals.borrow().is_empty() {
                        
                            html! {
                                <div class="m-7 w-full h-96">
                                    { "Please open/create a file or connect to a device!" }
                                </div>
                            }
                        
                        } else {
                        
                            html! {
                            <>
                                <div class="m-7 grid gap-5 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6">

                                    <MySelect
                                        prm_dat_distinct = { &MODE }
                                        vval={
                                            self.vvals.borrow().get_by_id(MODE.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />

                                    <MyInput
                                        prm_dat_dec = { &UL_PERIOD }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(UL_PERIOD.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />

                                    <MyInput
                                        prm_dat_dec = { &LORA_PERIOD }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(LORA_PERIOD.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />

                                    <MyInput
                                        prm_dat_dec = { &SOS_PERIOD }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(SOS_PERIOD.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />

                                    <MyOptionalInput
                                        prm_dat_optional = { &PERIODIC_POS_PERIOD }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(PERIODIC_POS_PERIOD.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />
                                    
                                    <div hidden = { !is_accelerometer_used } >
                                        <MyInput
                                            prm_dat_dec = { &MOTION_DURATION }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(MOTION_DURATION.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_accelerometer_used } class = "col-span-1 row-span-2">
                                        <div>
                                            <MycMotionSensitivity
                                                vval={
                                                    self.vvals.borrow().get_by_id(MOTION_SENSITIVITY.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                                />
                                        </div>
                                    </div>

                                    
                                    <div hidden = { !is_accelerometer_used } >
                                        <MyInput
                                            prm_dat_dec = { &MOTION_NB_POS }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(MOTION_NB_POS.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>
                                    
                                    <MyOptionalInput
                                        prm_dat_optional = { &SHOCK_DETECTION }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(SHOCK_DETECTION.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />
                            

                                    <div hidden = { is_standby } >
                                        <MySelect
                                            prm_dat_distinct = { &GEOLOC_SENSOR } 
                                            vval={
                                                self.vvals.borrow().get_by_id(GEOLOC_SENSOR.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    
                                    <MySelect
                                        prm_dat_distinct = { &GEOLOC_METHOD } 
                                        vval={
                                            self.vvals.borrow().get_by_id(GEOLOC_METHOD.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />

                                    <MySelect
                                        prm_dat_distinct = { &TRANSMIT_STRAT } 
                                        vval={
                                            self.vvals.borrow().get_by_id(TRANSMIT_STRAT.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
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
                                            vval={
                                                self.vvals.borrow().get_by_id(TRANSMIT_STRAT_CUSTOM.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <MyBitmap
                                        prm_dat_bitmap = { &CONFIG_FLAGS }
                                        vval={
                                            self.vvals.borrow().get_by_id(CONFIG_FLAGS.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />

                                    <div hidden = { !is_gps_used } >
                                        <MySelect
                                            prm_dat_distinct = { &GNSS_CONSTELLATION } 
                                            vval={
                                                self.vvals.borrow().get_by_id(GNSS_CONSTELLATION.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_gps_used } >
                                        <MyOptionalInput
                                            prm_dat_optional = { &GPS_T0_TIMEOUT }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(GPS_T0_TIMEOUT.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_gps_used } >
                                        <MyOptionalInput
                                            prm_dat_optional = { &GPS_T0_TIMEOUT_MOTION }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(GPS_T0_TIMEOUT_MOTION.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_gps_used } >
                                        <MyInput
                                            prm_dat_dec = { &GPS_EHPE }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(GPS_EHPE.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_gps_used } >
                                        <MyInput
                                            prm_dat_dec = { &GPS_EHPE_MOTION }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(GPS_EHPE_MOTION.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_gps_used } >
                                        <MyInput
                                            prm_dat_dec = { &GPS_CONVERGENCE }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(GPS_CONVERGENCE.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_gps_used } >
                                        <MyInput
                                            prm_dat_dec = { &GPS_CONVERGENCE_MOTION }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(GPS_CONVERGENCE_MOTION.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_gps_used } >
                                        <MyOptionalInput
                                            prm_dat_optional = { &GPS_FIX_TIMEOUT }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(GPS_FIX_TIMEOUT.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_gps_used } >
                                        <MyInput
                                            prm_dat_dec = { &GPS_TIMEOUT }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(GPS_TIMEOUT.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_gps_used } >
                                        <MyOptionalInput
                                            prm_dat_optional = { &GPS_STANDBY_TIMEOUT }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(GPS_STANDBY_TIMEOUT.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_ble_position_used } >
                                        <MyInput
                                            prm_dat_dec = { &BLE_BEACON_TIMEOUT }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(BLE_BEACON_TIMEOUT.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_ble_position_used } >
                                        <MyInput
                                            prm_dat_dec = { &BLE_RSSI_FILTER }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(BLE_RSSI_FILTER.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_ble_position_used } >
                                        <MyInput
                                            prm_dat_dec = { &BLE_BEACON_CNT }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(BLE_BEACON_CNT.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_ble_position_used } >
                                        <MySelect
                                            prm_dat_distinct = { &POSITION_BLE_FILTER_TYPE } 
                                            vval={
                                                self.vvals.borrow().get_by_id(POSITION_BLE_FILTER_TYPE.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_ble_position_used } >
                                        <MySelect
                                            prm_dat_distinct = { &POSITION_BLE_REPORT_TYPE } 
                                            vval={
                                                self.vvals.borrow().get_by_id(POSITION_BLE_REPORT_TYPE.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_ble_position_used } >
                                        <MyOptionalInput
                                            prm_dat_optional = { &POSITION_BLE_FILTER_MAIN_1 }
                                            radix_disp = { RadixDisp::Hex }
                                            vval={
                                                self.vvals.borrow().get_by_id(POSITION_BLE_FILTER_MAIN_1.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_ble_position_used } >
                                        <MyOptionalInput
                                            prm_dat_optional = { &POSITION_BLE_FILTER_MAIN_2 }
                                            radix_disp = { RadixDisp::Hex }
                                            vval={
                                                self.vvals.borrow().get_by_id(POSITION_BLE_FILTER_MAIN_2.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_ble_position_used } >
                                        <MyOptionalInput
                                            prm_dat_optional = { &POSITION_BLE_FILTER_SEC_VALUE }
                                            radix_disp = { RadixDisp::Hex }
                                            vval={
                                                self.vvals.borrow().get_by_id(POSITION_BLE_FILTER_SEC_VALUE.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <div hidden = { !is_ble_position_used } >
                                        <MyOptionalInput
                                            prm_dat_optional = { &POSITION_BLE_FILTER_SEC_MASK }
                                            radix_disp = { RadixDisp::Hex }
                                            vval={
                                                self.vvals.borrow().get_by_id(POSITION_BLE_FILTER_SEC_MASK.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
                                    </div>

                                    <MyInput
                                        prm_dat_dec = { &BUZZER_VOLUME }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(BUZZER_VOLUME.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />




                                    <div class = "col-span-1 row-span-2">
                                        <MycBatteryCapacity
                                            vval={
                                                self.vvals.borrow().get_by_id(BATTERY_CAPACITY.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                            />
                                    </div>

                                    <div class = "col-span-1 row-span-4">
                                        <MycButtonMapping
                                            vval={
                                                self.vvals.borrow().get_by_id(BUTTON_MAPPING.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                            />
                                    </div>

                                </div>

                                // -- end of PARAMETER COOMPONENTS

                                <div class = "m-7" >

                                    <div class = "felx flex-row justify-end gap-1 w-full">
                                        <button
                                            class = "m-2 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                                            onclick = { ctx.link().callback(move |_| {
                                                Msg::ShowCfg
                                            }) } 
                                        >
                                            { "Show Config Params" }
                                        </button>
                                        <button
                                            class = "m-2 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                                            onclick = { ctx.link().callback(move |_| {
                                                Msg::ShowLWDL
                                            }) } 
                                        >
                                            { "Show LoRaWAN DL Payloads" }
                                        </button>
                                    </div>

                                    <div class = "p-4">
                                        <pre>{ &self.greet_msg }</pre>
                                    </div>

                                </div>

                            </>
                            }
                        }
                    }

                </main>
            </>
        )
    }

}

