use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use gloo::console::log;

use crate::components::my_label::MyLabel;
use crate::params::{
    ParamType,
    param_id::ParamId,
    param_comp_type_props::ParamBitmapBit,
    param_values::{ParamValue, ValueUpdateData},
};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: ParamId,
    pub label: &'static str,
    pub description: &'static str,
    pub items: &'static [ParamBitmapBit],
    pub value: ParamValue,
    pub handle_onchange: Callback<ValueUpdateData>,
}


#[function_component(MyTransmitStratCustom)]
pub fn my_transmit_strat_custom(props: &Props) -> Html {

    let bitmap = match props.value {
        ParamValue::Valid(v) => v,
        _ => 0  // TODO: select the right default value!
    };

    let handle_onchange = props.handle_onchange.clone();
    let id = props.id;

    let select_id1 = format!("{}-select1", &props.id);
    let select_id2 = format!("{}-select2", &props.id);

    let on_select_change = {
        let select_id1 = select_id1.clone();
        let handle_onchange = handle_onchange.clone();
        Callback::from(move |event: Event| {

            let element = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>();

            let element_id = element.id(); 

            let value = element.value();
            let value: ParamType = value.parse().unwrap(); // TODO!

            let new_bitmap = if element_id == select_id1 {
                ( (value & 0b111)) << 2 | (bitmap & !(0b111 << 2) ) 
            } else {
                ( (value & 0b111)) << 5 | (bitmap & !(0b111 << 5) )
            };

            log!("value:", new_bitmap.to_string());
            handle_onchange.emit(ValueUpdateData{new_param_value: ParamValue::Valid(new_bitmap), param_id: id});

        })
    };


    let on_checkbox_change = {
        Callback::from(move |event: Event| {
            let checkbox = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>();

            checkbox.checked();
            let bitnum: usize = checkbox.value().parse().unwrap_or_default(); // TODO!
            let new_bitmap = if checkbox.checked() {
                (1 << bitnum) | bitmap
            } else {
                (!(1 << bitnum)) & bitmap
            };
            
            log!("value:", new_bitmap.to_string());
            handle_onchange.emit(ValueUpdateData{new_param_value: ParamValue::Valid(new_bitmap), param_id: id});
        })
    };

    let aria_select_id1 = format!("{}-aria_select1", &props.id);
    let aria_select_id2 = format!("{}-aria_select2", &props.id);

    let select_value1 = (bitmap >> 2) & 0b111;
    let select_value2 = (bitmap >> 5) & 0b111;

    let button_id1 = format!("{}-button1", &props.id);
    let button_id2 = format!("{}-button2", &props.id);
    let aria_button_id1 = format!("{}-aria_button1", &props.id);
    let aria_button_id2 = format!("{}-aria_button2", &props.id);
    let dropdown_id1 = format!("{}-dropdown1", props.id);
    let dropdown_id2 = format!("{}-dropdown2", props.id);


    // let two_checkboxes = {
    //     let item0_bitnumber = props.items[0].bit_number;
    //     let item0_description = props.items[0].description;
    //     let item1_bitnumber = props.items[1].bit_number;
    //     let item1_description = props.items[1].description;
    //     html! {<>

    //         <input 
    //             id={format!("{}-checkbox-{}", props.id, item0_bitnumber)} 
    //             type="checkbox"
    //             checked = { (*bitmap_state >> item0_bitnumber) & 1 == 1 } 
    //             value={item0_bitnumber.to_string()}
    //             onchange={on_checkbox_change.clone()}
    //             class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500"
    //         />
    //         <label for={format!("{}-checkbox-{}", props.id, item0_bitnumber)}
    //                 class="w-full ms-2 text-sm font-medium text-gray-900 rounded dark:text-gray-300">
    //             {item0_description}
    //         </label>

    //     </>}
    // };

    html! {
        <div>

            <MyLabel
                input_element_id={props.id}
                label={props.label}
                description={props.description}
                is_valid=true
            />
            <input id={props.id.to_string()} hidden=true />

            <div class={"ml-5"}>

                <div class={"mb-3"}>

                    <ul class="p-0 overflow-y-auto text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdownSearchButton">
                        {
                            props.items
                            .iter()
                            .filter(|item| {
                                item.bit_number == 0 || item.bit_number == 1
                            })
                            .map(|item| {
                                html!{

                                    <li>
                                        <div class="flex items-center px-2 py-1 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                                            <input 
                                                id={format!("{}-checkbox-{}", props.id, item.bit_number)} 
                                                type="checkbox"
                                                checked = { (bitmap >> item.bit_number) & 1 == 1 } 
                                                value={item.bit_number.to_string()}
                                                onchange={on_checkbox_change.clone()}
                                                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500"
                                            />
                                            <label for={format!("{}-checkbox-{}", props.id, item.bit_number)}
                                                    class="w-full ms-2 text-sm font-medium text-gray-900 rounded dark:text-gray-300">
                                                {item.description}
                                            </label>
                                        </div>
                                    </li>

                                }
                            })
                            .collect::<Html>()
                        }
                    </ul>

                </div>





                <div>

                    <div class={"mb-3"}>

                        <label for={ button_id1.clone() } class={"my-valid-label"}>
                            { "Data rates used for the 1st TX" }
                        </label>
                        // This button looks like an input field
                        <button 
                            type="button"
                            id={button_id1.clone()}
                            aria-describedby={aria_button_id1.clone()}
                            data-dropdown-toggle={dropdown_id1.clone()}
                            class="flex justify-between items-center bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-1 focus:ring-blue-500 focus:border-blue-500 block w-56 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
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
                                    props.items
                                    .iter()
                                    .filter(|item| {
                                        8 <= item.bit_number && item.bit_number <= 15
                                    })
                                    .map(|item| {
                                        html!{

                                            <li>
                                                <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                                                    <input 
                                                        id={format!("{}-checkbox-{}", props.id, item.bit_number)} 
                                                        type="checkbox"
                                                        checked = { (bitmap >> item.bit_number) & 1 == 1 } 
                                                        value={item.bit_number.to_string()}
                                                        onchange={on_checkbox_change.clone()}
                                                        class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500"
                                                    />
                                                    <label for={format!("{}-checkbox-{}", props.id, item.bit_number)}
                                                        class="w-full ms-2 text-sm font-medium text-gray-900 rounded dark:text-gray-300">
                                                        {item.description}
                                                    </label>
                                                </div>
                                            </li>

                                        }
                                    })
                                    .collect::<Html>()
                                }

                            </ul>
                        </div>
                        <span id={aria_button_id1} class="hidden">{&props.description}</span>

                    </div>


                    <div class={"mb-3"}>

                        <label for={ select_id1.clone() } class={"my-valid-label"}>
                            { "Data rate distribution of the 1st TX" }
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
                        <span id={aria_select_id1} class="hidden">{&props.description}</span>

                    </div>

                </div>


                <div hidden={ bitmap & 0b10 != 0b10 } >

                    <div class={"mb-3"}>
                        <label for={ button_id2.clone() } class={"my-valid-label"} >
                            { "Data rates used for the 2nd TX" }
                        </label>
                        // This button looks like an input field
                        <button 
                            type="button"
                            id={button_id2.clone()}
                            aria-describedby={aria_button_id2.clone()}
                            data-dropdown-toggle={dropdown_id2.clone()}
                            class="flex justify-between items-center bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-1 focus:ring-blue-500 focus:border-blue-500 block w-56 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
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
                                    props.items
                                    .iter()
                                    .filter(|item| {
                                        16 <= item.bit_number && item.bit_number <= 23
                                    })
                                    .map(|item| {
                                        html!{

                                            <li>
                                                <div class="flex items-center p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-600">
                                                    <input 
                                                        id={format!("{}-checkbox-{}", props.id, item.bit_number)} 
                                                        type="checkbox"
                                                        checked = { (bitmap >> item.bit_number) & 1 == 1 } 
                                                        value={item.bit_number.to_string()}
                                                        onchange={on_checkbox_change.clone()}
                                                        class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-700 dark:focus:ring-offset-gray-700 focus:ring-2 dark:bg-gray-600 dark:border-gray-500"
                                                    />
                                                    <label for={format!("{}-checkbox-{}", props.id, item.bit_number)}
                                                        class="w-full ms-2 text-sm font-medium text-gray-900 rounded dark:text-gray-300">
                                                        {item.description}
                                                    </label>
                                                </div>
                                            </li>

                                        }
                                    })
                                    .collect::<Html>()
                                }

                            </ul>
                        </div>
                        <span id={aria_button_id2} class="hidden">{&props.description}</span>
                    </div>

                    <div class={"mb-3"}>

                        <label for={ select_id2.clone() } class={"my-valid-label"} >
                            { "Data rate distribution of the 2nd TX" }
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
                        <span id={aria_select_id2} class="hidden">{&props.description}</span>
                    
                    </div>
                
                </div>

            </div>

        </div>
    }
}
