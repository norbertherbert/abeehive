use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::my_label::MyLabel;

use crate::prm::{
    typ::{ PrmDatBitmap, PrmDat, },
    val::PrmVVal,
};


#[derive(Properties, PartialEq)]
pub struct Props {

    pub prm_dat_bitmap: &'static PrmDatBitmap,

    // pub id: u8,
    // pub label: &'static str,
    // pub description: &'static str,
    // pub items: &'static [BitmapBit],

    pub vval: PrmVVal,
    pub handle_onchange: Callback<(u8, String)>,
}

#[function_component(MyBitmap)]
pub fn my_bitmap(props: &Props) -> Html {

    let bitmap = match props.vval {
        PrmVVal::Valid(v) => v,
        PrmVVal::Invalid((v, _)) => v,
        _ => 0, // TODO: select the right default value!
    };

    let handle_onchange = props.handle_onchange.clone();
    let id = props.prm_dat_bitmap.id;

    let on_checkbox_change = {
        Callback::from(move |event: Event| {
            let checkbox = event.target().unwrap().unchecked_into::<HtmlInputElement>();

            let bitnum: usize = checkbox.value().parse().unwrap_or_default(); // TODO!

            let new_bitmap = if checkbox.checked() {
                (1 << bitnum) | bitmap
            } else {
                (!(1 << bitnum)) & bitmap
            };

            log!("value:", new_bitmap.to_string());
            handle_onchange.emit((
                id,
                new_bitmap.to_string(),
            ));
        })
    };

    let vval = props.vval.clone();
    let aria_id = format!("{}-aria", &props.prm_dat_bitmap.id);
    let dropdown_id = format!("{}-dropdown", props.prm_dat_bitmap.id);

    html! {
        <div>

            <MyLabel
                prm_dat = { props.prm_dat_bitmap as &'static dyn PrmDat }
                is_valid = {
                    match &vval {
                        PrmVVal::Valid(_) => true,
                        PrmVVal::Invalid(_) => false,
                        PrmVVal::InvalidTxt(_) => false,
                    }
                }
            />

            // This button looks like an input field
            <button
                type="button"
                id={props.prm_dat_bitmap.id.to_string()}
                aria-describedby={aria_id.clone()}
                data-dropdown-toggle={dropdown_id.clone()}
                class = {
                    match &vval {
                        PrmVVal::Valid(_) => "my-valid-button",
                        PrmVVal::Invalid(_) => "my-invalid-button",
                        PrmVVal::InvalidTxt(_) => "my-invalid-button",
                    }
                }
            >
                {"Bitmap ..."}
                <svg class="w-2.5 h-2.5 ms-2.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                </svg>
            </button>

            // Dropdown menu
            <div
                id = { dropdown_id.clone() }
                // class = "my-bitmap-dropdown"
                class="hidden z-10 w-64 p-2.5 bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            >
                <ul class="h-48 p-0 overflow-y-auto text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdownSearchButton">

                    {
                        props.prm_dat_bitmap.bits.iter()
                        .filter_map(|item| {

                            let checked = (bitmap >> item.bit) & 1 == 1;

                            if !item.ena && !checked {
                                return None
                            }

                            Some(html!{

                                <li>
                                    <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                                        <input
                                            id = { format!("{}-checkbox-{}", props.prm_dat_bitmap.id, item.bit) }
                                            type = "checkbox"
                                            checked = { checked }
                                            value = { item.bit.to_string() }
                                            onchange = { on_checkbox_change.clone() }
                                            class = "w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500"
                                        />
                                        <label 
                                            for = { format!("{}-checkbox-{}", props.prm_dat_bitmap.id, item.bit) }
                                            class = {
                                                if item.ena {
                                                    // "w-full ms-2 text-sm font-medium text-gray-900 rounded dark:text-gray-300"
                                                    { "my-valid-checkbox-label"}
                                                } else {
                                                    // "w-full ms-2 text-sm font-medium  text-red-700 rounded dark:text-red-500;"
                                                    { "my-invalid-checkbox-label"}
                                                }
                                            }
                                        >
                                            { format!("{} - {}", item.bit, item.txt) }
                                        </label>
                                    </div>
                                </li>

                            })

                        }).collect::<Html>()
                    }

                </ul>
            </div>

            <span id={aria_id} class="hidden">{&props.prm_dat_bitmap.description}</span>

            // <span id={aria_id} class = "hidden">
            {
            match &vval {
                PrmVVal::Valid(_) => html!(),
                PrmVVal::Invalid((_, err)) => html!( <span class = { "my-error-msg" }>{ err.clone() }</span> ),
                PrmVVal::InvalidTxt((_, err)) => html!( <span class = { "my-error-msg" }>{ err.clone() }</span> ),
            }
            }
            // </span>

        </div>
    }
}
