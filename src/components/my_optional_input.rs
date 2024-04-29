use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use regex::Regex;
// use std::fmt;
// use gloo::console::log;

use crate::components::my_label::MyLabel;
use crate::params::{
    ParamType,
    param_id::ParamId,
    param_values::ValueUpdateData,
};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: ParamId,
    pub label: &'static str,
    pub description: &'static str,
    pub value: ParamType,
    pub disabled_value: ParamType,
    pub default_value: ParamType,
    pub valid_range: (ParamType, ParamType),
    pub handle_onchange: Callback<ValueUpdateData>,
}


#[derive(Clone, PartialEq)]
pub enum StateData {
    Valid(String),
    Invalid(String),
    Disabled
}

pub fn validate_num(valid_range: (ParamType, ParamType), value: ParamType) -> bool {
    if value < valid_range.0 || valid_range.1 < value {
        false
    } else {
        true
    }
}

const REGEX_STR: &str = "^[0-9]+$";
// const REGEX_STR: &str = "^[A-Fa-f0-9]{4}$";
const RADIX_NUM: u32 = 10;
// const RADIX_NUM: u32 = 16;
pub fn format_num(value_num: ParamType) -> String {
    format!("{}", value_num)
    // format!("{:04x}", value_num)
}

#[function_component(MyOptionalInput)]
pub fn my_optional_input(props: &Props) -> Html {

    let state = {
        let value_num = props.value;
        let valid_range = props.valid_range;
        use_state( move || {
            if props.value == props.disabled_value {
                StateData::Disabled
            } else {
                let is_valid = validate_num(valid_range, value_num);
                if is_valid {
                    StateData::Valid(format_num(value_num))
                } else {
                    StateData::Invalid(format_num(value_num))
                }
            }
        })
    };

    let checkbox_noderef = use_node_ref();
    let text_input_noderef = use_node_ref();

    let on_toggle = {

        let state = state.clone();

        let disabled_value = props.disabled_value;
        let default_value = props.default_value;
        let handle_onchange = props.handle_onchange.clone();
        let id = props.id;

        Callback::from(move |event: Event| {

            let checked = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .checked();

            // let text_input = text_input_noderef
            //     .cast::<web_sys::HtmlInputElement>()
            //     .unwrap();

            if checked {

                // text_input.set_value(&default_value.to_string());
                // text_input.set_disabled(false);
                // text_input.set_class_name(&enabled_classes);

                // let _ = text_input.focus();

                handle_onchange.emit(ValueUpdateData{new_value: default_value, param_id: id});
                state.set(StateData::Valid(default_value.to_string()));

            } else {

                // text_input.set_value(&disabled_value.to_string());
                // text_input.set_disabled(true);
                // text_input.set_class_name(&disabled_classes);
                
                handle_onchange.emit(ValueUpdateData{new_value: disabled_value, param_id: id});
                state.set(StateData::Disabled);

            }

        })
    };


    let onchange = {
    
        let state = state.clone();
        let handle_onchange = props.handle_onchange.clone();
        let valid_range = props.valid_range;
        let id = props.id; 

        Callback::from(move |event: Event| {

            let value_string = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            let re = Regex::new(REGEX_STR).unwrap();
            if !re.is_match(&value_string) {
                state.set( StateData::Invalid(value_string) );
                return;
            } 

            let Ok(value_num) = ParamType::from_str_radix(&value_string, RADIX_NUM) else {
                state.set( StateData::Invalid(value_string) );
                return;
            };

            if !validate_num(valid_range, value_num) {
                state.set( StateData::Invalid(format_num(value_num)) );
                return;
            } 

            handle_onchange.emit(ValueUpdateData{new_value: value_num, param_id: id});
            state.set( StateData::Valid(format_num(value_num)) );

        })
    };


    let aria_id = format!("{}-aria", &props.id);
    let checkbox_id = format!("{}-checkbox", &props.id);

    html! {
        <div>

            <MyLabel
                input_element_id={props.id}
                label={props.label}
                description={props.description}
                is_valid= { 
                    match &*state { 
                        StateData::Valid(_) => true,
                        StateData::Invalid(_) => false,
                        StateData::Disabled => true,
                    } 
                }
            />

            // Checkbox + Input field
            <div class="relative">

                // Checkbox (on top of the input field)
                <div class="absolute inset-y-0 start-0 flex items-center ps-2.5">
                    <label class="inline-flex items-center cursor-pointer">
                        <input
                            type="checkbox" 
                            id={checkbox_id}
                            ref={checkbox_noderef}
                            class="sr-only peer"
                            checked={ 
                                match &*state { 
                                    StateData::Valid(_) => true,
                                    StateData::Invalid(_) => true,
                                    StateData::Disabled => false
                                } 
                            } 
                            onchange={on_toggle}
                        />
                        <div 
                            class="relative w-9 h-5 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"
                        ></div>
                    </label>
                </div>

                // Input field
                <input 
                    type="text"
                    autocomplete="off"
                    id={props.id.to_string()}
                    ref={text_input_noderef}
                    class={ 
                        match &*state { 
                            StateData::Valid(_) => classes!("ps-14", "my-valid-optional-input"),
                            StateData::Invalid(_) => classes!("ps-14", "my-invalid-optional-input"),
                            StateData::Disabled => classes!("ps-14", "my-disabled-optional-input")
                        } 
                    } 
                    value={
                        match &*state { 
                            StateData::Valid(v) => v.to_string().clone(),
                            StateData::Invalid(v) => v.to_string().clone(),
                            StateData::Disabled => props.disabled_value.to_string().clone(),
                        } 
                    }
                    disabled={ 
                        match &*state { 
                            StateData::Valid(_) => false,
                            StateData::Invalid(_) => false,
                            StateData::Disabled => true
                        } 
                    } 
                    aria-describedby={aria_id.clone()}
                    onchange={onchange}
                />

                // Aria
                <span id={aria_id} class="hidden">{&props.description}</span>

            </div>

        </div>
    }
}