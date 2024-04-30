use yew::prelude::*;
// use yewdux::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
// use std::fmt;

use crate::components::my_label::MyLabel;
use crate::params::{
    ParamType,
    param_id::ParamId,
    param_comp_type_props::ParamOptionVariant,
    param_values::{ParamValue, ValueUpdateData},
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: ParamId,
    pub label: &'static str,
    pub description: &'static str,
    pub select_options: &'static [ParamOptionVariant],
    pub value: ParamValue,
    pub handle_onchange: Callback<ValueUpdateData>,
}

#[function_component(MySelect)]
pub fn my_select(props: &Props) -> Html {

    // let (param_values_state, _) = use_store::<ParamValues>();

    let handle_onchange = props.handle_onchange.clone();
    let id = props.id;

    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        
        let value: ParamType = value.parse().unwrap(); // TODO!
        let new_param_value = ParamValue::Valid(value);

        handle_onchange.emit(ValueUpdateData{new_param_value, param_id: id});
    });


    let aria_id = format!("{}-aria", &props.id);
    let value = match props.value {
        ParamValue::Valid(v) => v,
        _ => props.select_options[0].value  // TODO: select the right default value!
    };


    html! {
        <div>

            <MyLabel
                input_element_id={props.id}
                label={props.label}
                description={props.description}
                is_valid=true
            />

            <select 
                id={props.id.to_string()} 
                
                value={value.to_string()}

                onchange={onchange} 
                aria-describedby={aria_id.clone()} 
                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            >

                { 
                    props.select_options.iter().map(|item| {
                        html!{ <option value={item.value.to_string()} selected={item.value == value} >{item.text}</option> }
                    }).collect::<Html>()
                }

            </select>

            <span id={aria_id} class="hidden">{&props.description}</span>

        </div>
    }
}