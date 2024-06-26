use gloo::console::log;
// use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::my_label::MyLabel;

use crate::prm::{
    val::PrmVVal,
    dat::BATTERY_CAPACITY,
    typ::PrmDat,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub vval: PrmVVal,
    pub handle_onchange: Callback<(u8, String)>,
}

#[function_component(MycBatteryCapacity)]
pub fn myc_battery_capacity(props: &Props) -> Html {

    let battery_type_ref = NodeRef::default();
    let battery_capacity_ref = NodeRef::default();

    let set_default = {
        let handle_onchange = props.handle_onchange.clone();
        let id = BATTERY_CAPACITY.id;
        let default_val = BATTERY_CAPACITY.default_val;
        Callback::from(move |_event: MouseEvent| {
            let val = default_val;
            log!(&format!("{}", val));
            handle_onchange.emit((id, format!("{}", val)));
        })
    };

    let onchange = {

        let handle_onchange = props.handle_onchange.clone();

        let battery_type_ref = battery_type_ref.clone();
        let battery_capacity_ref = battery_capacity_ref.clone();
    
        let id = BATTERY_CAPACITY.id;

        let (default_val_battery_type, default_val_battery_capacity) = match BATTERY_CAPACITY.default_val {
            -1 | 0 => { (BATTERY_CAPACITY.default_val, 43500) },
            1..=65535 => { (1, BATTERY_CAPACITY.default_val) },
            _ => { (-1, 0) }
        };

        Callback::from(move |_event: Event| {
            
            // let txt = event
            //     .target()
            //     .unwrap()
            //     .unchecked_into::<HtmlInputElement>()
            //     .value();

            let battery_type_i32 = battery_type_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<i32>()
                .unwrap_or(default_val_battery_type);

            let mut battery_capacity_i32 = battery_capacity_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<i32>()
                .unwrap_or(default_val_battery_capacity);

            if battery_capacity_i32 < 1 { battery_capacity_i32 = 1 }
            if battery_capacity_i32 > 65535 { battery_capacity_i32 = 65535 }

            let val = match battery_type_i32 {
                -1 | 0 => battery_type_i32,
                1 => battery_capacity_i32,
                _ => -1,
            };

            log!(format!("{:08x}", val));
            handle_onchange.emit((id, val.to_string()));

        })
    };

    let vval = props.vval.clone();

    let (
        battery_type, 
        battery_capacity, 
        err 
    ) = match vval {
        PrmVVal::Valid(val) => {
            match val {
                -1 | 0 => { ( val.to_string() , 43500.to_string(), "".to_string() ) }
                1..=65535 => { ( 1.to_string() , val.to_string(), "".to_string() ) }
                _ => { ( (-1).to_string() , 43500.to_string(), "".to_string() ) }
            }
        }
        PrmVVal::Invalid((val, err)) => {
            match val {
                -1 | 0 => { ( val.to_string() , 43500.to_string(), err ) }
                1..=65535 => { ( 1.to_string() , val.to_string(), err ) }
                _ => { ( (-1).to_string() , 43500.to_string(), err ) }
            }
        }
        PrmVVal::InvalidTxt((val, err)) => {
            (
                "".to_string(), 
                "".to_string(), 
                format!("cannot decode value: {}; {}", val, err)
            )
        }
    };

    html! {
  
        <div>

            <MyLabel
                prm_dat = { &BATTERY_CAPACITY as &'static dyn PrmDat }
                is_valid = { err.is_empty() }
            />

            {

                if !err.is_empty() { html!(


                    <div class = { "my-invalid-div" } >

                        { "Invalid values." } 
    
                        <button
                            class = "ml-auto text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                            onclick = { set_default }
                        >
                            { "Use default" }
                        </button>
    
                    </div>

                )} else { html!(

                    <div 
                        class = {
                            if err.is_empty() { "my-vertical-div" }
                            else { "my-invalid-div" }
                        }
                    >

                        <div>
                            <label for = { BATTERY_CAPACITY.id.to_string() } >
                                {"Battery Type"}
                            </label>
                            <select
                                id = { BATTERY_CAPACITY.id.to_string() }
                                class = {
                                    if err.is_empty() { "my-valid-input" }
                                    else { "my-invalid-input" }
                                }
                                value = { battery_type.clone() }

                                ref = { battery_type_ref.clone() }
                                onchange = { onchange.clone() }
                            >
                                {
                                    BATTERY_CAPACITY.distinct_vals.iter().map(|item| {
                                        html!{ 
                                            <option value = { item.val.to_string() } selected = { item.val.to_string() == battery_type } >
                                                // { format!("{} - {}", item.val, item.txt) }
                                                { item.txt }
                                            </option> 
                                        }
                                    }).collect::<Html>()
                                }
                            </select>
                        </div>

                        <div hidden = {battery_type != "1"} >
                            <label>{"Capacity [mAh]"}</label>
                            <input
                                type="text"
                                autocomplete = "off"
                                class = {
                                    if err.is_empty() { "my-valid-input" }
                                    else { "my-invalid-input" }
                                }
                                value = { battery_capacity.clone() }
                                ref = { battery_capacity_ref.clone() }
                                onchange={ onchange.clone() }
                            />
                        </div>

                    </div>

                )}

            }

        </div>
    }

}

    
