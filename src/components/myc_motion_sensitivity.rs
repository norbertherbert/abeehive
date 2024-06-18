use gloo::console::log;
// use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::my_label::MyLabel;

// use crate::prm::dat::MotionSensFsOptions;
use crate::prm::{
    typ::{
        PrmVal, 
        DistinctVal
    },
    val::PrmVVal,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: u8,
    pub label: &'static str,
    pub description: &'static str,

    pub default_val: PrmVal,
    pub range_sensitivity: (u8, u8),
    pub distinct_vals_odr: &'static [DistinctVal],
    pub distinct_vals_fullscale: &'static [DistinctVal],

    pub vval: PrmVVal,
    pub handle_onchange: Callback<(u8, String)>,
}

#[function_component(MycMotionSensitivity)]
pub fn myc_motion_sensitivity(props: &Props) -> Html {

    let sensitivity_ref = NodeRef::default();
    let odr_ref = NodeRef::default();
    let fullscale_ref = NodeRef::default();

    let set_default = {
        let handle_onchange = props.handle_onchange.clone();
        let id = props.id;
        let default_val = props.default_val;
        Callback::from(move |_event: MouseEvent| {
            let val = default_val;
            log!(&format!("{:08x}", val));
            handle_onchange.emit((id, format!("{}", val)));
        })
    };

    let onchange = {
        let handle_onchange = props.handle_onchange.clone();
        let sensitivity_ref = sensitivity_ref.clone();
        let odr_ref = odr_ref.clone();
        let fullscale_ref = fullscale_ref.clone();
        let id = props.id;
        Callback::from(move |_event: Event| {
            
            // let txt = event
            //     .target()
            //     .unwrap()
            //     .unchecked_into::<HtmlInputElement>()
            //     .value();


            let mut sensitivity_i32 = sensitivity_ref
            .cast::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<i32>()
            .unwrap_or(63);

            let odr_i32 =odr_ref
            .cast::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<i32>()
            .unwrap_or(0);

            let fullscale_i32 = fullscale_ref
            .cast::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<i32>()
            .unwrap_or(0);

            let min_encoded_sens_val = 
                if fullscale_i32 == 2 { 2 }
                else if fullscale_i32 == 3 { 4 }
                else { 1 };
            
            sensitivity_i32 = sensitivity_i32 / 63 + (sensitivity_i32 % 63).signum();
            if sensitivity_i32 < min_encoded_sens_val { sensitivity_i32 = min_encoded_sens_val; }
            if sensitivity_i32 > 30 { sensitivity_i32 = 30; }

            let val = sensitivity_i32 + (odr_i32 << 8) + (fullscale_i32 << 16);
            log!(format!("{:08x}", val));
            handle_onchange.emit((id, format!("{}", val)));

        })
    };


    let vval = props.vval.clone();

    let (sensitivity, odr, fullscale, err ) = match vval {
        PrmVVal::Valid(val) => {
            let val = val.to_le_bytes();
            (
                (val[0] as i32 * 63).to_string(), val[1].to_string(), val[2].to_string(), "".to_string()
            )
        }
        PrmVVal::Invalid((val, err)) => {
            let val = val.to_le_bytes();
            (
                (val[0] as i32 * 63).to_string(), val[1].to_string(), val[2].to_string(), err
            )
        }
        PrmVVal::InvalidTxt((val, err)) => {
            (
                "".to_string(), "".to_string(), "".to_string(), 
                format!("cannot decode value: {}; {}", val, err)
            )
        }
    };


    html! {
  
        <div>

            <MyLabel
                input_element_id = { props.id }
                label = { props.label }
                description = { props.description}
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
                            <label>{"Sensitivity [mg]"}</label>
                            <input
                                type="text"
                                autocomplete = "off"
                                class = {
                                    if err.is_empty() { "my-valid-input" }
                                    else { "my-invalid-input" }
                                }
                                value = { sensitivity.clone() }
                                ref = { sensitivity_ref.clone() }
                                onchange={ onchange.clone() }
                            />
                        </div>


                        <div class = "flex gap-2 justify-items-stretch">

                            <div class = "flex-1">
                                <label>{"Sample Rate"}</label>
                                <select
                                    class = {
                                        if err.is_empty() { "my-valid-input" }
                                        else { "my-invalid-input" }
                                    }
                                    value = { odr.clone() }

                                    ref = { odr_ref.clone() }
                                    onchange = { onchange.clone() }
                                >
                                    {
                                        props.distinct_vals_odr.iter().map(|item| {
                                            html!{ 
                                                <option value = { item.val.to_string() } selected = { item.val.to_string() == odr } >
                                                    // { format!("{} - {}", item.val, item.txt) }
                                                    { item.txt }
                                                </option> 
                                            }
                                        }).collect::<Html>()
                                    }
                                </select>
                            </div>

                            <div class = "flex-1">
                                <label>{"Full Scale"}</label>
                                <select
                                    class = {
                                        if err.is_empty() { "my-valid-input" }
                                        else { "my-invalid-input" }
                                    }
                                    value = { fullscale.clone() }
                                    ref = { fullscale_ref.clone() }
                                    onchange = { onchange }
                                >
                                    {
                                        props.distinct_vals_fullscale.iter().map(|item| {
                                            html!{ 
                                                <option value = { item.val.to_string() } selected = { item.val.to_string() == fullscale } >
                                                    // { format!("{} - {}", item.val, item.txt) }
                                                    { item.txt }
                                                </option> 
                                            }
                                        }).collect::<Html>()
                                    }
                                </select>

                            </div>
                        </div>
                    </div>

                )}

            }

        </div>
    }

}
