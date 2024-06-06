use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::my_label::MyLabel;

use crate::params::param_values::{ParamValue, ValueUpdateData};
use crate::prm::typ::BitmapBit;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: u8,
    pub label: &'static str,
    pub description: &'static str,
    pub items: &'static [BitmapBit],
    pub value: ParamValue,
    pub handle_onchange: Callback<ValueUpdateData>,
}

#[function_component(MyBitmap)]
pub fn my_bitmap(props: &Props) -> Html {
    let bitmap = match props.value {
        ParamValue::Valid(v) => v,
        _ => 0, // TODO: select the right default value!
    };

    let handle_onchange = props.handle_onchange.clone();
    let id = props.id;

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
            handle_onchange.emit(ValueUpdateData {
                new_param_value: ParamValue::Valid(new_bitmap),
                param_id: id,
            });
        })
    };

    let aria_id = format!("{}-aria", &props.id);
    let dropdown_id = format!("{}-dropdown", props.id);

    html! {
        <div>

            <MyLabel
                input_element_id={props.id}
                label={props.label}
                description={props.description}
                is_valid=true
            />

            // This button looks like an input field
            <button
                type="button"
                id={props.id.to_string()}
                aria-describedby={aria_id.clone()}
                data-dropdown-toggle={dropdown_id.clone()}
                class="flex justify-between items-center bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-1 focus:ring-blue-500 focus:border-blue-500 block w-56 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            >
                {"Bitmap ..."}
                <svg class="w-2.5 h-2.5 ms-2.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                </svg>
            </button>

            // Dropdown menu
            <div
                id={dropdown_id.clone()}
                class="z-10 hidden bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-56 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            >
                <ul class="h-48 p-0 overflow-y-auto text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdownSearchButton">

                    {
                        props.items.iter().map(|item| {

                            html!{

                                <li>
                                    <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                                        <input
                                            id={format!("{}-checkbox-{}", props.id, item.bit)}
                                            type="checkbox"
                                            checked = { (bitmap >> item.bit) & 1 == 1 }
                                            value={item.bit.to_string()}
                                            onchange={on_checkbox_change.clone()}
                                            class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500"
                                        />
                                        <label for={format!("{}-checkbox-{}", props.id, item.bit)}
                                             class="w-full ms-2 text-sm font-medium text-gray-900 rounded dark:text-gray-300">
                                            {item.txt}
                                        </label>
                                    </div>
                                </li>

                            }
                        }).collect::<Html>()
                    }

                </ul>
            </div>

            <span id={aria_id} class="hidden">{&props.description}</span>

        </div>
    }
}
