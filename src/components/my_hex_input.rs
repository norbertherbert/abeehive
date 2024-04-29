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
    pub valid_range: (ParamType, ParamType),
    pub handle_onchange: Callback<ValueUpdateData>,
}

#[derive(Clone, PartialEq)]
pub enum StateData {
    Valid(String),
    Invalid(String),
}

pub fn validate_num(valid_range: (ParamType, ParamType), value: ParamType) -> bool {
    if value < valid_range.0 || valid_range.1 < value {
        false
    } else {
        true
    }
}


// const REGEX_STR: &str = "^[0-9]+$";
const REGEX_STR: &str = "^[A-Fa-f0-9]{4}$";
// const RADIX_NUM: u32 = 10;
const RADIX_NUM: u32 = 16;
pub fn format_num(value_num: ParamType) -> String {
    // format!("{}", value_num)
    format!("{:04x}", value_num)
}


#[function_component(MyHexInput)]
pub fn my_hex_input(props: &Props) -> Html {
    
    let state = {
        let value_num = props.value;
        let valid_range = props.valid_range;
        use_state( move || {
            let is_valid = validate_num(valid_range, value_num);
            if is_valid {
                StateData::Valid(format_num(value_num))
            } else {
                StateData::Invalid(format_num(value_num))
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
                    } 
                }
            />

            <input 
                type="text"
                autocomplete="off"
                id={props.id.to_string()}
                class={ 
                    match &*state { 
                        StateData::Valid(_) => "my-valid-input",
                        StateData::Invalid(_) => "my-invalid-input",
                    } 
                } 
                value={
                    match &*state { 
                        StateData::Valid(v) => v.to_string().clone(),
                        StateData::Invalid(v) => v.to_string().clone(),
                    } 
                }
                aria-describedby={aria_id.clone()} 
                onchange={onchange} 
            />

            <span id={aria_id} class="hidden">{&props.description}</span>

        </div>
    }

}