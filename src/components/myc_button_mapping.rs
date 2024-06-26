use gloo::console::log;
// use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::my_label::MyLabel;

use crate::prm::{
    val::PrmVVal,
    dat::BUTTON_MAPPING,
    typ::PrmDat,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub vval: PrmVVal,
    pub handle_onchange: Callback<(u8, String)>,
}

#[function_component(MycButtonMapping)]
pub fn myc_button_mapping(props: &Props) -> Html {

    let long_press_action_ref = NodeRef::default();
    let short_press_action_ref = NodeRef::default();
    let short_press2_action_ref = NodeRef::default();
    let short_press3_action_ref = NodeRef::default();
    let long_press_duration_ref = NodeRef::default();

    let set_default = {
        let handle_onchange = props.handle_onchange.clone();
        let id = BUTTON_MAPPING.id;
        let default_val = BUTTON_MAPPING.default_val;
        Callback::from(move |_event: MouseEvent| {
            let val = default_val;
            log!(&format!("{:08x}", val));
            handle_onchange.emit((id, format!("{}", val)));
        })
    };

    let onchange = {

        let handle_onchange = props.handle_onchange.clone();

        let long_press_action_ref = long_press_action_ref.clone();
        let short_press_action_ref = short_press_action_ref.clone();
        let short_press2_action_ref = short_press2_action_ref.clone();
        let short_press3_action_ref = short_press3_action_ref.clone();
        let long_press_duration_ref = long_press_duration_ref.clone();

        let id = BUTTON_MAPPING.id;
        let default_val = BUTTON_MAPPING.default_val;

        Callback::from(move |_event: Event| {
            
            // let txt = event
            //     .target()
            //     .unwrap()
            //     .unchecked_into::<HtmlInputElement>()
            //     .value();

            let long_press_action_i32 = long_press_action_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<i32>()
                .unwrap_or(default_val & 0xf);

            let short_press_action_i32 = short_press_action_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<i32>()
                .unwrap_or((default_val>>4) & 0xf);

            let short_press2_action_i32 = short_press2_action_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<i32>()
                .unwrap_or((default_val>>8) & 0xf);

            let short_press3_action_i32 = short_press3_action_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<i32>()
                .unwrap_or((default_val>>12) & 0xf);

            let mut long_press_duration_i32 = long_press_duration_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<i32>()
                .unwrap_or((default_val>>16) & 0xf);

            if long_press_duration_i32 < 1 { long_press_duration_i32 = 1 }
            if long_press_duration_i32 > 8 { long_press_duration_i32 = 8 }

            let val = 
                long_press_action_i32 + 
                (short_press_action_i32 << 4) + 
                (short_press2_action_i32 << 8) + 
                (short_press3_action_i32 << 12) + 
                (long_press_duration_i32 << 16); 

            log!(format!("{:08x}", val));
            handle_onchange.emit((id, val.to_string()));

        })
    };

    let vval = props.vval.clone();

    let (
        long_press_action, 
        short_press_action, 
        short_press2_action, 
        short_press3_action,
        long_press_duration,
        err 
    ) = match vval {
        PrmVVal::Valid(val) => {
            (
                (val & 0xf).to_string(), 
                ((val>>4) & 0xf).to_string(), 
                ((val>>8) & 0xf).to_string(), 
                ((val>>12) & 0xf).to_string(), 
                ((val>>16) & 0xf).to_string(), 
                "".to_string()
            )
        }
        PrmVVal::Invalid((val, err)) => {
            (
                (val & 0xf).to_string(), 
                ((val>>4) & 0xf).to_string(), 
                ((val>>8) & 0xf).to_string(), 
                ((val>>12) & 0xf).to_string(), 
                ((val>>16) & 0xf).to_string(), 
                err
            )
        }
        PrmVVal::InvalidTxt((val, err)) => {
            (
                "".to_string(), 
                "".to_string(), 
                "".to_string(),
                "".to_string(),
                "".to_string(), 
                format!("cannot decode value: {}; {}", val, err)
            )
        }
    };

    html! {
  
        <div>

            <MyLabel
                prm_dat = { &BUTTON_MAPPING as &'static dyn PrmDat }
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
                            <label for = { BUTTON_MAPPING.id.to_string() } >
                                {"Long Press Action"}
                            </label>
                            <select
                                id = { BUTTON_MAPPING.id.to_string() }
                                class = {
                                    if err.is_empty() { "my-valid-input" }
                                    else { "my-invalid-input" }
                                }
                                value = { long_press_action.clone() }

                                ref = { long_press_action_ref.clone() }
                                onchange = { onchange.clone() }
                            >
                                {
                                    BUTTON_MAPPING.action_distinct_vals.iter().map(|item| {
                                        html!{ 
                                            <option value = { item.val.to_string() } selected = { item.val.to_string() == long_press_action } >
                                                // { format!("{} - {}", item.val, item.txt) }
                                                { item.txt }
                                            </option> 
                                        }
                                    }).collect::<Html>()
                                }
                            </select>
                        </div>

                        <div>
                            <label>{"Long Press Duration [s]"}</label>
                            <input
                                type="text"
                                autocomplete = "off"
                                class = {
                                    if err.is_empty() { "my-valid-input" }
                                    else { "my-invalid-input" }
                                }
                                value = { long_press_duration.clone() }
                                ref = { long_press_duration_ref.clone() }
                                onchange={ onchange.clone() }
                            />
                        </div>

                        <div>
                            <label>{"Short Press (Click) Action"}</label>
                            <select
                                class = {
                                    if err.is_empty() { "my-valid-input" }
                                    else { "my-invalid-input" }
                                }
                                value = { short_press_action.clone() }

                                ref = { short_press_action_ref.clone() }
                                onchange = { onchange.clone() }
                            >
                                {
                                    BUTTON_MAPPING.action_distinct_vals.iter().map(|item| {
                                        html!{ 
                                            <option value = { item.val.to_string() } selected = { item.val.to_string() == short_press_action } >
                                                // { format!("{} - {}", item.val, item.txt) }
                                                { item.txt }
                                            </option> 
                                        }
                                    }).collect::<Html>()
                                }
                            </select>
                        </div>


                        <div>
                            <label>{"Double-click Action"}</label>
                            <select
                                class = {
                                    if err.is_empty() { "my-valid-input" }
                                    else { "my-invalid-input" }
                                }
                                value = { short_press2_action.clone() }

                                ref = { short_press2_action_ref.clone() }
                                onchange = { onchange.clone() }
                            >
                                {
                                    BUTTON_MAPPING.action_distinct_vals.iter().map(|item| {
                                        html!{ 
                                            <option value = { item.val.to_string() } selected = { item.val.to_string() == short_press2_action } >
                                                // { format!("{} - {}", item.val, item.txt) }
                                                { item.txt }
                                            </option> 
                                        }
                                    }).collect::<Html>()
                                }
                            </select>
                        </div>

                        <div>
                            <label>{"Tripple-click Action"}</label>
                            <select
                                class = {
                                    if err.is_empty() { "my-valid-input" }
                                    else { "my-invalid-input" }
                                }
                                value = { short_press3_action.clone() }

                                ref = { short_press3_action_ref.clone() }
                                onchange = { onchange.clone() }
                            >
                                {
                                    BUTTON_MAPPING.action_distinct_vals.iter().map(|item| {
                                        html!{ 
                                            <option value = { item.val.to_string() } selected = { item.val.to_string() == short_press3_action } >
                                                // { format!("{} - {}", item.val, item.txt) }
                                                { item.txt }
                                            </option> 
                                        }
                                    }).collect::<Html>()
                                }
                            </select>
                        </div>

                    </div>

                )}

            }

        </div>
    }

}

    
