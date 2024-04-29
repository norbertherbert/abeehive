use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
// use std::fmt;

use crate::components::my_label::MyLabel;
use crate::params::{
    ParamType,
    param_id::ParamId,
    param_options1::ParamOption,
    // param_comp_type_props::ParamOptionVariant,
    param_values::ValueUpdateData,
};

#[derive(Properties, PartialEq)]
pub struct Props<T> 
    where T: 'static + PartialEq,
{
    pub id: ParamId,
    pub label: &'static str,
    pub description: &'static str,
    pub select_options: &'static [T],
    pub value: ParamType,
    pub handle_onchange: Callback<ValueUpdateData>,
}

#[function_component(MySelect1)]
pub fn my_select1<T>(props: &Props<T>) -> Html 
    where T: 'static + PartialEq + Clone + std::fmt::Display + ParamOption,
{

    let handle_onchange = props.handle_onchange.clone();
    let id = props.id;

    let onchange = Callback::from(move |event: Event| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        let value: ParamType = value.parse().unwrap(); // TODO!
        handle_onchange.emit(ValueUpdateData{new_value: value, param_id: id});
    });

    let aria_id = format!("{}-aria", &props.id);

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
                value={props.value.clone().to_string()}
                onchange={onchange} 
                aria-describedby={aria_id.clone()} 
                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            >

                { 
                    props.select_options.iter().map(|item| {
                        html!{ <option value={item.as_value().to_string()} selected={item.as_value() == props.value} >{item.to_string()}</option> }
                    }).collect::<Html>()
                }

            </select>

            <span id={aria_id} class="hidden">{&props.description}</span>

        </div>
    }
}