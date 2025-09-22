use std::cell::RefCell;
use std::rc::Rc;
use web_sys::HtmlElement;

use wasm_bindgen_futures::JsFuture;

use abeehive::prm::val::PrmVVals;
// use std::fmt;
// use yewdux::prelude::*;
use gloo::console::log;
// use js_sys::{
//     // Function, 
//     Function, Promise
// };
use serde::Deserialize; // , Serialize};
// use serde_wasm_bindgen::to_value;
// use std::ops::Deref;
use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use abeehive::js::file_api;

use abeehive::components::{
    RadixDisp,
    my_input::MyInput,
    my_optional_input::MyOptionalInput,
    my_select::MySelect,
    my_bitmap::MyBitmap,
    myc_transmit_strat_custom::MycTransmitStratCustom,
    myc_motion_sensitivity::MycMotionSensitivity,
    myc_button_mapping::MycButtonMapping,
    myc_battery_capacity::MycBatteryCapacity,

    modal::Modal,

    navbar::{Navbar, NavbarAction},
};

use abeehive::prm::{ 
    dat::*, val::PrmVVal, 
    // typ::PrmVal 
};

use abeehive::templates::*;


#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_name = "initFlowbite")]
    fn init_flowbite();

}


#[derive(Debug, PartialEq)]
pub enum Msg {
    ParamValueChanged(u8, String),
    ShowCfg,
    ShowLWDL,
    New(CfgTemplate),
    Close,
    Open,
    AboutModalToggle,
    // OpenWith,
    SaveAs,
    UpdateVVals((String, PrmVVals)),
    UpdateSourceName(String),
    Navbar(NavbarAction),
}

#[derive(Default, Debug)]
pub struct BeeQueenApp {
    source_name: String,
    vvals: Rc<RefCell<PrmVVals>>,
    about_modal_is_visible: bool,
    greet_msg: String,
    // file_open_listener: Option<(Promise, Closure<dyn FnMut(JsValue)>)>,
    // save_as_listener: Option<(Promise, Closure<dyn FnMut(JsValue)>)>,
    phantom_node_ref: NodeRef, // needed for simulating click outside a Flowbite dropdown and make it close
}


#[derive(Deserialize, Debug, Clone, PartialEq)]
struct FileOpenResult {
    file_name: String,
    config_str: String,
}


impl Component for BeeQueenApp {
    type Message = Msg;
    type Properties = ();
  
    fn create(_ctx: &Context<Self>) -> Self {

        // let beequeen_app = BeeQueenApp::default();

        let beequeen_app = BeeQueenApp {
            source_name: String::new(),
            vvals: Rc::new(RefCell::new(PrmVVals::new())),
            about_modal_is_visible: false,
            greet_msg: String::new(),
            // file_open_listener: None,
            // save_as_listener: None,
            phantom_node_ref: NodeRef::default(),
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

            Msg::New(templ) => {
                
                // let vvals = PrmVVals::default();

                if let Ok(vvals) = PrmVVals::from_cfg_str(templ.txt()) {
                    self.source_name = format!("Template: {}", templ.name());
                    // let vvals = PrmVVals::default();
                    self.vvals.replace(vvals);
                } else {
                    self.source_name = format!("Template: Cannot Open Template!");
                    let vvals = PrmVVals::new();
                    self.vvals.replace(vvals);
                }                
                true
            },

            Msg::Close => {
                let vvals = PrmVVals::new();
                self.vvals.replace(vvals);
                self.source_name = String::new();
                true
            },

            Msg::Open => {

                let update_on_file_open = ctx.link().callback(move | args: (String, PrmVVals)| {
                    Msg::UpdateVVals(args)
                });

                wasm_bindgen_futures::spawn_local(async move {
                    match JsFuture::from(file_api::open_config_file_str()).await {
                        Ok(js_val) => {
                            // Deserialize the returned JS object into our Rust struct
                            let res: Result<FileOpenResult, _> = serde_wasm_bindgen::from_value(js_val);
                            match res {
                                Ok(data) => {
                                    // log!(&data.file_name);
                                    // log!(&data.config_str);

                                    let res  = PrmVVals::from_cfg_str(&data.config_str);
                                    match res {
                                        Ok(vvals) => update_on_file_open.emit((data.file_name, vvals)),
                                        Err(e) => log!(format!("Parameter parse error: {e:?}")),
                                    };

                                }
                                Err(e) => log!(format!("Deserialize error: {e:?}")),
                            }
                        }
                        Err(e) => log!(format!("JS error: {e:?}")),
                    }
                });
                
                true
            },
            Msg::AboutModalToggle => {
                self.about_modal_is_visible = !self.about_modal_is_visible;
                true
            },
            Msg::SaveAs => {

                let file_name = "config.txt";
                let config_str = self.vvals.borrow_mut().to_cfg_string();
                // log!(config_str.clone());

                wasm_bindgen_futures::spawn_local(async move {
                    match JsFuture::from(file_api::save_config_file_str(file_name, &config_str)).await {
                        Ok(js_val) => {
                            // Deserialize the returned JS object into our Rust struct
                            let res: Result<String, _> = serde_wasm_bindgen::from_value(js_val);
                            match res {
                                Ok(file_name) => {
                                    log!(file_name);
                                }
                                Err(e) => log!(format!("Deserialize error: {e:?}")),
                            }
                        }
                        Err(e) => log!(format!("JS error: {e:?}")),
                    }
                });

                true
            },
            
            Msg::UpdateVVals(( source_name, vvals )) => {
                // log!(format!("{:?}", vvals));
                self.source_name = source_name;
                self.vvals.replace(vvals);
                self.about_modal_is_visible = false;
                true
            }

            Msg::UpdateSourceName( source_name) => {
                self.source_name = source_name;
                true
            }

            Msg::Navbar(navbar_action) => {
                if let Some(element) = self.phantom_node_ref.cast::<HtmlElement>() {
                    element.click();
                }
                match navbar_action {

                    NavbarAction::New(templ) => {
                        ctx.link().callback(move |_: ()| {
                            Msg::New(templ.clone())
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
                    NavbarAction::AboutModalShow => {
                        ctx.link().callback(move |_: ()| {
                            Msg::AboutModalToggle
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


    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {

        // if first_render { 

        //     let open_with = ctx.link().callback(move |_| {
        //         Msg::OpenWith
        //     });
        //     open_with.emit(());

        // }

        // ************************************************************************
        // *** Initiating the Flowbite framework as an external Javascript Call 
        // ************************************************************************

        init_flowbite();
       
    }

    // fn destroy(&mut self, _ctx: &Context<Self>) {

    //     if let Some(ref file_open_listener) = self.file_open_listener {
        
    //         let unlisten_file_open_promise = file_open_listener.0.clone();
        
    //         spawn_local(async move {

    //             let unlisten_file_open: Function = wasm_bindgen_futures::JsFuture::from(unlisten_file_open_promise)
    //             .await
    //             .unwrap()
    //             .into();

    //             unlisten_file_open.call0(&JsValue::undefined()).unwrap();

    //         });

    //         self.file_open_listener = None;

    //     }

    //     if let Some(ref save_as_listener) = self.save_as_listener {
        
    //         let unlisten_save_as_promise = save_as_listener.0.clone();
        
    //         spawn_local(async move {

    //             let unlisten_save_as: Function = wasm_bindgen_futures::JsFuture::from(unlisten_save_as_promise)
    //             .await
    //             .unwrap()
    //             .into();

    //             unlisten_save_as.call0(&JsValue::undefined()).unwrap();

    //         });

    //         self.save_as_listener = None;

    //     }

    // }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let handle_onchange = {
            ctx.link().callback(move |(id, txt)| {
                Msg::ParamValueChanged(id, txt)
            })
        };

        let mut is_standby = true;
        let mut is_geoloc_used = true;
        let mut is_gps_used = false; 
        let mut is_lpgps_used = false;
        let mut is_ble_positioning_used = false;
        let mut _is_wifi_used = false;
        let mut is_accelerometer_used = false;

        let is_shock_detection_on;
        let mut is_ble_beaconing_on = false;
        let mut is_scan_collection_on = false;
        let mut is_geofencing_on = false;
        let mut is_angle_detection_on = false;

        if !self.vvals.borrow().is_empty() {

            is_shock_detection_on = match self.vvals.borrow().get_by_id(SHOCK_DETECTION.id)
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

            is_ble_beaconing_on = match self.vvals.borrow().get_by_id(BEACONING_TYPE.id)
                .expect("id is always valid")
                .expect("id always exists")
            {
                PrmVVal::Valid(v) => {
                    match v {
                        v if *v==BeaconingTypeOptions::DISABLED.val => false,
                        _ => true,
                    }
                },
                _ => false
            };

            is_scan_collection_on = match self.vvals.borrow().get_by_id(COLLECTION_SCAN_TYPE.id)
                .expect("id is always valid")
                .expect("id always exists")
            {
                PrmVVal::Valid(v) => {
                    match v {
                        v if *v==CollScanTypeOption::NONE.val => false,
                        _ => true,
                    }
                },
                _ => false
            };

            is_geofencing_on = match self.vvals.borrow().get_by_id(GEOFENCING_SCAN_PERIOD.id)
                .expect("id is always valid")
                .expect("id always exists")
            {
                PrmVVal::Valid(v) => {
                    match v {
                        v if *v==GEOFENCING_SCAN_PERIOD.disabled_val => false,
                        _ => true,
                    }
                },
                _ => false
            };

            is_angle_detection_on = match self.vvals.borrow().get_by_id(ANGLE_DETECT_MODE.id)
                .expect("id is always valid")
                .expect("id always exists")
            {
                PrmVVal::Valid(v) => {
                    match v {
                        v if *v==AngleDetectModeOptions::DISABLED.val => false,
                        _ => true,
                    }
                },
                _ => false
            };

            (is_accelerometer_used, is_standby, is_geoloc_used) = 
                match self.vvals.borrow().get_by_id(MODE.id)
                    .expect("id is always valid")
                    .expect("id always exists")
                {
                    PrmVVal::Valid(v) => {
                        match v {
                            v if *v==ModeOption::STANDBY.val => (false, true, false),
                            v if *v==ModeOption::MOTION_TRACKING.val => (true, false, true),
                            v if *v==ModeOption::PERMANENT_TRACKING.val => (false, false, true),
                            v if *v==ModeOption::MOTION_START_END_TRACKING.val => (true, false, true),
                            v if *v==ModeOption::ACTIVITY_TRACKING.val => (true, false, false),
                            _ => (false, false, false)
                        }
                    },
                    _ => (false, false, false)
                };

            is_accelerometer_used |= is_shock_detection_on;

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
                (is_gps_used, is_lpgps_used, is_ble_positioning_used, _is_wifi_used) = 
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

            let (is_gps_used_1, is_lpgps_used_1, is_ble_positioning_used_1, is_wifi_used_1) = 
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

                        let is_ble_positioning_used = match v {
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

                        (is_gps_used, is_lpgps_used, is_ble_positioning_used, is_wifi_used)

                    },
                    _ => (false, false, false, false) 
                };

            is_gps_used |= is_gps_used_1;
            is_lpgps_used |= is_lpgps_used_1;
            is_ble_positioning_used |= is_ble_positioning_used_1;
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

                <main class="pt-14" ref={self.phantom_node_ref.clone()} >

                    <Modal
                        title = "About BeeQueen"
                        is_visible = { self.about_modal_is_visible }
                        onclose = { ctx.link().callback(move |_| {
                            Msg::AboutModalToggle
                        }) } 
                    >

                        { "BeeQueen AT2v2.6 - Abeeway Configuration Editor Tool for Asset Tracker v2 firmware v2.6" }

                        // <button
                        //     class="mt-5 text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        //     onclick = { ctx.link().callback(move |_| {
                        //         Msg::SaveConfigUsb
                        //     }) } 
                        // >
                        //     { "Save Config to Device" }
                        // </button>

                    </Modal>

                    

                    // -- PARAMETER COOMPONENTS:

                    {
                        if self.vvals.borrow().is_empty() {
                        
                            html! {
                                <div class="m-7 w-full h-96">
                                    { "Please open/create a file or connect a device!" }
                                </div>
                            }
                        
                        } else {
                        
                            html! {
                            <>
                                <div class="m-7 grid gap-5 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6">

                                    // Main Operating Mode Parameters
                                    <h5 class="col-span-full text-xl font-bold dark:text-white">
                                        {"Main Operating Mode Parameters"}
                                    </h5>

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

                                    <div hidden = { is_standby | !is_geoloc_used } >
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
                                    
                                    <div hidden = { is_standby } >
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
                                    </div>

                                    // Side Operating Mode Parameters
                                    <h5 class="col-span-full text-xl font-bold dark:text-white">
                                        {"Side Operating Mode Parameters"}
                                    </h5>


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
                                    
                                    <MyOptionalInput
                                        prm_dat_optional = { &PERIODIC_ACTIVITY_PERIOD }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(PERIODIC_ACTIVITY_PERIOD.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />

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

                                    // Angle Detection Parameters
                                    <div class = {
                                        if is_angle_detection_on {
                                            "col-span-full grid gap-5 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6"
                                        }
                                        else {""}                                          
                                    }>
                                        <h7 class="col-span-full text-base font-bold dark:text-white">
                                            {"Angle Detection Parameters"}
                                        </h7>

                                        <MySelect
                                            prm_dat_distinct = { &ANGLE_DETECT_MODE } 
                                            vval={
                                                self.vvals.borrow().get_by_id(ANGLE_DETECT_MODE.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />

                                        <div hidden={!is_angle_detection_on}>
                                            <MySelect
                                                prm_dat_distinct = { &ANGLE_REF_ACQ } 
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_REF_ACQ.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_angle_detection_on}>
                                            <MyOptionalInput
                                                prm_dat_optional = { &ANGLE_REF_ACC_X }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_REF_ACC_X.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_angle_detection_on}>
                                            <MyOptionalInput
                                                prm_dat_optional = { &ANGLE_REF_ACC_Y }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_REF_ACC_Y.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_angle_detection_on}>
                                            <MyOptionalInput
                                                prm_dat_optional = { &ANGLE_REF_ACC_Z }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_REF_ACC_Z.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_angle_detection_on}>
                                            <MyInput
                                                prm_dat_dec = { &ANGLE_CRITICAL }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_CRITICAL.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            /> 
                                        </div>                                  
                                        
                                        <div hidden={!is_angle_detection_on}>
                                            <MyInput
                                                prm_dat_dec = { &ANGLE_CRITICAL_HYST }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_CRITICAL_HYST.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>


                                        <div hidden={!is_angle_detection_on}>
                                            <MyBitmap
                                                prm_dat_bitmap = { &ANGLE_REPORT_MODE }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_REPORT_MODE.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>


                                        <div hidden={!is_angle_detection_on}>
                                            <MyInput
                                                prm_dat_dec = { &ANGLE_REPORT_PERIOD }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_REPORT_PERIOD.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_angle_detection_on}>
                                            <MyInput
                                                prm_dat_dec = { &ANGLE_REPORT_REPEAT }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_REPORT_REPEAT.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_angle_detection_on}>
                                            <MyInput
                                                prm_dat_dec = { &ANGLE_RISING_TIME }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_RISING_TIME.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_angle_detection_on}>
                                            <MyInput
                                                prm_dat_dec = { &ANGLE_FALLING_TIME }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_FALLING_TIME.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_angle_detection_on}>
                                            <MyInput
                                                prm_dat_dec = { &ANGLE_LEARNING_TIME }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_LEARNING_TIME.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_angle_detection_on}>
                                            <MyInput
                                                prm_dat_dec = { &ANGLE_ACC_ACCURACY }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_ACC_ACCURACY.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_angle_detection_on}>
                                            <MyInput
                                                prm_dat_dec = { &ANGLE_DEVIATION_DELTA }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_DEVIATION_DELTA.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_angle_detection_on}>
                                            <MyInput
                                                prm_dat_dec = { &ANGLE_DEVIATION_MIN_INTERVAL }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_DEVIATION_MIN_INTERVAL.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_angle_detection_on}>
                                            <MyInput
                                                prm_dat_dec = { &ANGLE_DEVIATION_MAX_INTERVAL }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(ANGLE_DEVIATION_MAX_INTERVAL.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>
                                    </div>




                                    // BLE Beaconing Parameters
                                    <div class = {
                                        if is_ble_beaconing_on {
                                            "col-span-full grid gap-5 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6"
                                        }
                                        else {""}                                          
                                    }>
                                        <h7 class="col-span-full text-base font-bold dark:text-white">
                                            {"BLE Beaconing Parameters"}
                                        </h7>
                                        
                                        <MySelect
                                            prm_dat_distinct = { &BEACONING_TYPE } 
                                            vval={
                                                self.vvals.borrow().get_by_id(BEACONING_TYPE.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />

                                        <div hidden={!is_ble_beaconing_on}>
                                            <MySelect
                                                prm_dat_distinct = { &BEACONING_TX_POWER } 
                                                vval={
                                                    self.vvals.borrow().get_by_id(BEACONING_TX_POWER.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_ble_beaconing_on}>
                                            <MyOptionalInput
                                                prm_dat_optional = { &BEACONING_STATIC_INTERVAL }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(BEACONING_STATIC_INTERVAL.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_ble_beaconing_on}>
                                            <MyOptionalInput
                                                prm_dat_optional = { &BEACONING_MOTION_INTERVAL }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(BEACONING_MOTION_INTERVAL.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_ble_beaconing_on}>
                                            <MyInput
                                                prm_dat_dec = { &BEACONING_MOTION_DURATION }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(BEACONING_MOTION_DURATION.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_ble_beaconing_on}>
                                            <MyInput
                                                prm_dat_dec = { &BEACON_ID_0 }
                                                radix_disp = { RadixDisp::Hex }
                                                vval={
                                                    self.vvals.borrow().get_by_id(BEACON_ID_0.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_ble_beaconing_on}>
                                            <MyInput
                                                prm_dat_dec = { &BEACON_ID_1 }
                                                radix_disp = { RadixDisp::Hex }
                                                vval={
                                                    self.vvals.borrow().get_by_id(BEACON_ID_1.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_ble_beaconing_on}>
                                            <MyInput
                                                prm_dat_dec = { &BEACON_ID_2 }
                                                radix_disp = { RadixDisp::Hex }
                                                vval={
                                                    self.vvals.borrow().get_by_id(BEACON_ID_2.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_ble_beaconing_on}>
                                            <MyInput
                                                prm_dat_dec = { &BEACON_ID_3 }
                                                radix_disp = { RadixDisp::Hex }
                                                vval={
                                                    self.vvals.borrow().get_by_id(BEACON_ID_3.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_ble_beaconing_on}>
                                            <MyInput
                                                prm_dat_dec = { &BEACON_ID_4 }
                                                radix_disp = { RadixDisp::Hex }
                                                vval={
                                                    self.vvals.borrow().get_by_id(BEACON_ID_4.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                    </div>




                                    // Scan Collection Parameters
                                    <div class = {
                                        if is_scan_collection_on {
                                            "col-span-full grid gap-5 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6"
                                        }
                                        else {""}                                          
                                    }>
                                        <h7 class="col-span-full text-base font-bold dark:text-white">
                                            {"Scan Collection Parameters"}
                                        </h7>

                                        <MySelect
                                            prm_dat_distinct = { &COLLECTION_SCAN_TYPE } 
                                            vval={
                                                self.vvals.borrow().get_by_id(COLLECTION_SCAN_TYPE.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />

                                        <div hidden={!is_scan_collection_on}>
                                            <MyInput
                                                prm_dat_dec = { &COLLECTION_NB_ENTRY }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(COLLECTION_NB_ENTRY.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_scan_collection_on}>
                                            <MySelect
                                                prm_dat_distinct = { &COLLECTION_BLE_FILTER_TYPE } 
                                                vval={
                                                    self.vvals.borrow().get_by_id(COLLECTION_BLE_FILTER_TYPE.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_scan_collection_on}>
                                            <MyOptionalInput
                                                prm_dat_optional = { &COLLECTION_BLE_FILTER_MAIN_1 }
                                                radix_disp = { RadixDisp::Hex }
                                                vval={
                                                    self.vvals.borrow().get_by_id(COLLECTION_BLE_FILTER_MAIN_1.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_scan_collection_on}>
                                            <MyOptionalInput
                                                prm_dat_optional = { &COLLECTION_BLE_FILTER_MAIN_2 }
                                                radix_disp = { RadixDisp::Hex }
                                                vval={
                                                    self.vvals.borrow().get_by_id(COLLECTION_BLE_FILTER_MAIN_2.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_scan_collection_on}>
                                            <MyOptionalInput
                                                prm_dat_optional = { &COLLECTION_BLE_FILTER_SEC_VALUE }
                                                radix_disp = { RadixDisp::Hex }
                                                vval={
                                                    self.vvals.borrow().get_by_id(COLLECTION_BLE_FILTER_SEC_VALUE.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_scan_collection_on}>
                                            <MyOptionalInput
                                                prm_dat_optional = { &COLLECTION_BLE_FILTER_SEC_MASK }
                                                radix_disp = { RadixDisp::Hex }
                                                vval={
                                                    self.vvals.borrow().get_by_id(COLLECTION_BLE_FILTER_SEC_MASK.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                    </div>
                                    
                                    // BLE Geofencing Parameters
                                    <div class = {
                                        if is_geofencing_on {
                                            "col-span-full grid gap-5 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6"
                                        }
                                        else {""}                                          
                                    }>
                                        <h7 class="col-span-full text-base font-bold dark:text-white">
                                            {"BLE Geofencing Parameters"}
                                        </h7>

                                        <MyOptionalInput
                                            prm_dat_optional = { &GEOFENCING_SCAN_PERIOD }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(GEOFENCING_SCAN_PERIOD.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        /> 

                                        <div hidden={!is_geofencing_on}>
                                            <MyInput
                                                prm_dat_dec = { &GEOFENCING_COLLECT_PERIOD }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(GEOFENCING_COLLECT_PERIOD.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                        <div hidden={!is_geofencing_on}>
                                            <MyInput
                                                prm_dat_dec = { &GEOFENCING_SCAN_DURATION }
                                                radix_disp = { RadixDisp::Dec }
                                                vval={
                                                    self.vvals.borrow().get_by_id(GEOFENCING_SCAN_DURATION.id)
                                                    .expect("id is always valid")
                                                    .expect("id always exists")
                                                    .clone()
                                                }
                                                handle_onchange = { handle_onchange.clone() }
                                            />
                                        </div>

                                    </div>


                                    // GNSS Configuration Parameters
                                    <h5 hidden = { !is_gps_used } class="col-span-full text-xl font-bold dark:text-white">
                                        {"GNSS Configuration Parameters"}
                                    </h5> 

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

                                    <div hidden = { !is_lpgps_used } >
                                        <MyInput
                                            prm_dat_dec = { &AGPS_TIMEOUT }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(AGPS_TIMEOUT.id)
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


                                    // BLE Positioning Configuration Parameters
                                    <h5 hidden = { !is_ble_positioning_used } class="col-span-full text-xl font-bold dark:text-white">
                                        {"BLE Positioning Configuration Parameters"}
                                    </h5> 

                                    <div hidden = { !is_ble_positioning_used } >
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

                                    <div hidden = { !is_ble_positioning_used } >
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

                                    <div hidden = { !is_ble_positioning_used } >
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

                                    <div hidden = { !is_ble_positioning_used } >
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

                                    <div hidden = { !is_ble_positioning_used } >
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

                                    <div hidden = { !is_ble_positioning_used } >
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

                                    <div hidden = { !is_ble_positioning_used } >
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

                                    <div hidden = { !is_ble_positioning_used } >
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

                                    <div hidden = { !is_ble_positioning_used } >
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


                                    // Accelerometer Configuration Parameters
                                    <h5 hidden = { !is_accelerometer_used } class="col-span-full text-xl font-bold dark:text-white">
                                        {"Accelerometer Configuration Parameters"}
                                    </h5>                                    

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
                                            prm_dat_dec = { &MOTION_DEBOUNCE }
                                            radix_disp = { RadixDisp::Dec }
                                            vval={
                                                self.vvals.borrow().get_by_id(MOTION_DEBOUNCE.id)
                                                .expect("id is always valid")
                                                .expect("id always exists")
                                                .clone()
                                            }
                                            handle_onchange = { handle_onchange.clone() }
                                        />
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




                                    // LoRaWAN Transmission Parameters
                                    <h5 class="col-span-full text-xl font-bold dark:text-white">
                                        {"LoRaWAN Transmission Parameters"}
                                    </h5>

                                    <MySelect
                                        prm_dat_distinct = { &DEFAULT_DATARATE } 
                                        vval={
                                            self.vvals.borrow().get_by_id(DEFAULT_DATARATE.id)
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
                                        prm_dat_bitmap = { &CONFIRMED_UL_BITMAP }
                                        vval={
                                            self.vvals.borrow().get_by_id(CONFIRMED_UL_BITMAP.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />

                                    <MyInput
                                        prm_dat_dec = { &CONFIRMED_UL_RETRY }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(CONFIRMED_UL_RETRY.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />


                                    <MyOptionalInput
                                        prm_dat_optional = { &NETWORK_TIMEOUT_CHECK }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(NETWORK_TIMEOUT_CHECK.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />

                                    <MyOptionalInput
                                        prm_dat_optional = { &NETWORK_TIMEOUT_RESET }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(NETWORK_TIMEOUT_RESET.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />


                                    // Generic Configuration Parameters
                                    <h5 class="col-span-full text-xl font-bold dark:text-white">
                                        {"Generic Configuration Parameters"}
                                    </h5> 

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

                                    <MyOptionalInput
                                        prm_dat_optional = { &PW_STAT_PERIOD }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(PW_STAT_PERIOD.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />

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

                                    <MyInput
                                        prm_dat_dec = { &BLE_CNX_ADV_DURATION }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(BLE_CNX_ADV_DURATION.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />

                                    // Temperature Parameters

                                    <MyOptionalInput
                                        prm_dat_optional = { &TEMPERATURE_HIGH }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(TEMPERATURE_HIGH.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />
                                    <MyOptionalInput
                                        prm_dat_optional = { &TEMPERATURE_LOW }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(TEMPERATURE_LOW.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />
                                    <MySelect
                                        prm_dat_distinct = { &TEMPERATURE_ACTION } //
                                        vval={
                                            self.vvals.borrow().get_by_id(TEMPERATURE_ACTION.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />

                                    <MySelect
                                        prm_dat_distinct = { &REED_SWITCH_CONFIGURATION } //
                                        vval={
                                            self.vvals.borrow().get_by_id(REED_SWITCH_CONFIGURATION.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />

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

                                    <MyInput
                                        prm_dat_dec = { &PASSWORD }
                                        radix_disp = { RadixDisp::Dec }
                                        vval={
                                            self.vvals.borrow().get_by_id(PASSWORD.id)
                                            .expect("id is always valid")
                                            .expect("id always exists")
                                            .clone()
                                        }
                                        handle_onchange = { handle_onchange.clone() }
                                    />

                                </div>


                                // -- end of PARAMETER COMPONENTS

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

