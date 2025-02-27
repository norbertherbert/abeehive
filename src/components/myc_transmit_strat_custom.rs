use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
// use yew::classes;

use crate::components::my_label::MyLabel;

use crate::prm::{
    val::PrmVVal,
    dat::TRANSMIT_STRAT_CUSTOM,
    typ::{ PrmVal, PrmDat, },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub vval: PrmVVal,
    pub handle_onchange: Callback<(u8, String)>,
}

#[function_component(MycTransmitStratCustom)]
pub fn myc_transmit_strat_custom(props: &Props) -> Html {
    
    let bitmap = match props.vval {
        PrmVVal::Valid(v) => v,
        PrmVVal::Invalid((v, _)) => v,
        _ => 0, // TODO: select the right default value!
    };

    let handle_onchange = props.handle_onchange.clone();
    let id = TRANSMIT_STRAT_CUSTOM.id;

    let select_id1 = format!("{}-select1", &TRANSMIT_STRAT_CUSTOM.id);
    let select_id2 = format!("{}-select2", &TRANSMIT_STRAT_CUSTOM.id);

    let on_select_change = {
        let select_id1 = select_id1.clone();
        let handle_onchange = handle_onchange.clone();
        Callback::from(move |event: Event| {
            let element = event.target().unwrap().unchecked_into::<HtmlInputElement>();

            let element_id = element.id();

            let value = element.value();
            let value: PrmVal = value.parse().unwrap(); // TODO!

            let new_bitmap = if element_id == select_id1 {
                (value & 0b111) << 2 | (bitmap & !(0b111 << 2))
            } else {
                (value & 0b111) << 5 | (bitmap & !(0b111 << 5))
            };

            log!("value:", new_bitmap.to_string());
            handle_onchange.emit((
                id,
                new_bitmap.to_string(),
            ));
        })
    };

    let on_checkbox_change = {
        Callback::from(move |event: Event| {
            let checkbox = event.target().unwrap().unchecked_into::<HtmlInputElement>();

            checkbox.checked();
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

    let aria_select_id1 = format!("{}-aria_select1", &TRANSMIT_STRAT_CUSTOM.id);
    let aria_select_id2 = format!("{}-aria_select2", &TRANSMIT_STRAT_CUSTOM.id);

    let select_value1 = (bitmap >> 2) & 0b111;
    let select_value2 = (bitmap >> 5) & 0b111;

    let button_id1 = format!("{}-button1", &TRANSMIT_STRAT_CUSTOM.id);
    let button_id2 = format!("{}-button2", &TRANSMIT_STRAT_CUSTOM.id);
    let aria_button_id1 = format!("{}-aria_button1", &TRANSMIT_STRAT_CUSTOM.id);
    let aria_button_id2 = format!("{}-aria_button2", &TRANSMIT_STRAT_CUSTOM.id);
    let dropdown_id1 = format!("{}-dropdown1", TRANSMIT_STRAT_CUSTOM.id);
    let dropdown_id2 = format!("{}-dropdown2", TRANSMIT_STRAT_CUSTOM.id);

    html! {
        <div>

            <MyLabel
                prm_dat = { &TRANSMIT_STRAT_CUSTOM as &'static dyn PrmDat }
                is_valid = true
            />
            <input id={TRANSMIT_STRAT_CUSTOM.id.to_string()} hidden=true />

            <div 
                // class={"ml-5"}
                class = "my-vertical-div"
                // class = "flex flex-col items-stretch gap-2 w-full p-2.5 bg-gray-50 border border-gray-300 rounded-lg focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-blue-500 dark:focus:border-blue-500;"
            >


                // 2 checkboxes for bit 0 and bit 1

                <div>

                    <ul class="p-0 overflow-y-auto text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdownSearchButton">
                        {
                            TRANSMIT_STRAT_CUSTOM.bits
                            .iter()
                            .filter(|item| {
                                item.bit == 0 || item.bit == 1
                            })
                            .map(|item| {
                                html!{

                                    <li>
                                        <div class="flex items-center px-2 py-1 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                                            <input
                                                id={format!("{}-checkbox-{}", TRANSMIT_STRAT_CUSTOM.id, item.bit)}
                                                type="checkbox"
                                                checked = { (bitmap >> item.bit) & 1 == 1 }
                                                value={item.bit.to_string()}
                                                onchange={on_checkbox_change.clone()}
                                                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500"
                                            />
                                            <label for={format!("{}-checkbox-{}", TRANSMIT_STRAT_CUSTOM.id, item.bit)}
                                                    class="w-full ms-2 text-sm font-medium text-gray-900 rounded dark:text-gray-300">
                                                {item.txt}
                                            </label>
                                        </div>
                                    </li>

                                }
                            })
                            .collect::<Html>()
                        }
                    </ul>

                </div>


                <div class = "flex flex-wrap gap-2 justify-items-stretch">

                    // 1st TX Data Rates

                    <div class="shrink-0 grow basis-7">

                        <label for={ button_id1.clone() } class={"my-valid-label"}>
                            { "Data rates used for the 1st TX" }
                        </label>
                        // This button looks like an input field
                        <button
                            type="button"
                            id={button_id1.clone()}
                            aria-describedby={aria_button_id1.clone()}
                            data-dropdown-toggle={dropdown_id1.clone()}
                            // class="flex justify-between items-center bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-1 focus:ring-blue-500 focus:border-blue-500 block w-56 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            class="my-valid-button"
                        >
                            {"Bitmap ..."}
                            <svg class="w-2.5 h-2.5 ms-2.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                            </svg>
                        </button>

                        // Dropdown menu
                        <div
                            id={dropdown_id1.clone()}
                            class="z-10 hidden bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-56 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        >
                            <ul class="h-48 p-0 overflow-y-auto text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdownSearchButton">

                                {
                                    TRANSMIT_STRAT_CUSTOM.bits
                                    .iter()
                                    .filter(|item| {
                                        8 <= item.bit && item.bit <= 15
                                    })
                                    .map(|item| {
                                        html!{

                                            <li>
                                                <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                                                    <input
                                                        id={format!("{}-checkbox-{}", TRANSMIT_STRAT_CUSTOM.id, item.bit)}
                                                        type="checkbox"
                                                        checked = { (bitmap >> item.bit) & 1 == 1 }
                                                        value={item.bit.to_string()}
                                                        onchange={on_checkbox_change.clone()}
                                                        class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500"
                                                    />
                                                    <label for={format!("{}-checkbox-{}", TRANSMIT_STRAT_CUSTOM.id, item.bit)}
                                                        class="w-full ms-2 text-sm font-medium text-gray-900 rounded dark:text-gray-300">
                                                        {item.txt}
                                                    </label>
                                                </div>
                                            </li>

                                        }
                                    })
                                    .collect::<Html>()
                                }

                            </ul>
                        </div>
                        <span id={aria_button_id1} class="hidden">{&TRANSMIT_STRAT_CUSTOM.description}</span>

                    </div>

                    // 1st TX DR Distribution

                    <div class="shrink-0 grow basis-7">

                        <label for={ select_id1.clone() } class={"my-valid-label"}>
                            { "Data rate distr. of the 1st TX" }
                        </label>
                        <select
                            id={ select_id1 }
                            value={ select_value1.to_string() }
                            onchange={ on_select_change.clone() }
                            aria-describedby={ aria_select_id1.clone() }
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        >

                            <option value="0" selected={select_value1 == 0} >{"Random"}</option>
                            <option value="1" selected={select_value1 == 1} >{"Bell Curve"}</option>
                            <option value="2" selected={select_value1 == 2} >{"Ring"}</option>

                        </select>
                        <span id={aria_select_id1} class="hidden">{&TRANSMIT_STRAT_CUSTOM.description}</span>

                    </div>

                </div>
              

                <div
                    class = { 
                        if bitmap & 0b10 == 0b10 { 
                            "flex gap-2 justify-items-stretch" 
                        } else { 
                            "hidden" 
                        } 
                    } 
                >

                    // 2nd TX Data Rates

                    <div class="flex-1">
                        <label for={ button_id2.clone() } class={"my-valid-label"} >
                            { "Data rates used for the 2nd TX" }
                        </label>
                        // This button looks like an input field
                        <button
                            type="button"
                            id={button_id2.clone()}
                            aria-describedby={aria_button_id2.clone()}
                            data-dropdown-toggle={dropdown_id2.clone()}
                            // class="flex justify-between items-center bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-1 focus:ring-blue-500 focus:border-blue-500 block w-56 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            class="my-valid-button"
                        >
                            {"Bitmap ..."}
                            <svg class="w-2.5 h-2.5 ms-2.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                            </svg>
                        </button>

                        // Dropdown menu
                        <div
                            id={dropdown_id2.clone()}
                            class="z-10 hidden bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-56 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        >
                            <ul class="h-48 p-0 overflow-y-auto text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdownSearchButton">

                                {
                                    TRANSMIT_STRAT_CUSTOM.bits
                                    .iter()
                                    .filter(|item| {
                                        16 <= item.bit && item.bit <= 23
                                    })
                                    .map(|item| {
                                        html!{

                                            <li>
                                                <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                                                    <input
                                                        id={format!("{}-checkbox-{}", TRANSMIT_STRAT_CUSTOM.id, item.bit)}
                                                        type="checkbox"
                                                        checked = { (bitmap >> item.bit) & 1 == 1 }
                                                        value={item.bit.to_string()}
                                                        onchange={on_checkbox_change.clone()}
                                                        class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500"
                                                    />
                                                    <label for={format!("{}-checkbox-{}", TRANSMIT_STRAT_CUSTOM.id, item.bit)}
                                                        class="w-full ms-2 text-sm font-medium text-gray-900 rounded dark:text-gray-300">
                                                        {item.txt}
                                                    </label>
                                                </div>
                                            </li>

                                        }
                                    })
                                    .collect::<Html>()
                                }

                            </ul>
                        </div>
                        <span id={aria_button_id2} class="hidden">{&TRANSMIT_STRAT_CUSTOM.description}</span>
                    </div>

                    // 2nd TX DR Distribution

                    <div class="flex-1">

                        <label for={ select_id2.clone() } class={"my-valid-label"} >
                            { "Data rate distr. of the 2nd TX" }
                        </label>
                        <select
                            id={ select_id2 }
                            value={ select_value2.to_string() }
                            onchange={ on_select_change }
                            aria-describedby={ aria_select_id2.clone() }
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                        >

                            <option value="0" selected={select_value2 == 0} >{"Random"}</option>
                            <option value="1" selected={select_value2 == 1} >{"Bell Curve"}</option>
                            <option value="2" selected={select_value2 == 2} >{"Ring"}</option>

                        </select>
                        <span id={aria_select_id2} class="hidden">{&TRANSMIT_STRAT_CUSTOM.description}</span>

                    </div>

                </div>

            </div>

        </div>
    }
}
