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
    Valid(ParamType),
    Invalid(String),
    None
}

const REGEX_STR: &str = "^[0-9]+$";
// const REGEX_STR: &str = "^[A-Fa-f0-9]{4}$";
const RADIX_NUM: u32 = 10;
// const RADIX_NUM: u32 = 16;
pub fn format_num(value_num: ParamType) -> String {
    format!("{}", value_num)
    // format!("{:04x}", value_num)
}

pub fn validate_num(valid_range: (ParamType, ParamType), value_num: ParamType) -> StateData {
    if value_num < valid_range.0 || valid_range.1 < value_num {
        StateData::Invalid(format_num(value_num))
    } else {
        StateData::Valid(value_num)
    }
}

pub fn validate_str(valid_range: (ParamType, ParamType), value_str: &str) -> StateData {

    if value_str=="" {
        return StateData::None;
    } 

    let re = Regex::new(REGEX_STR).unwrap();
    if !re.is_match(value_str) {
        return StateData::Invalid(value_str.to_string());
    } 

    let Ok(value_num) = ParamType::from_str_radix(value_str, RADIX_NUM) else {
        return StateData::Invalid(value_str.to_string());
    };

    validate_num(valid_range, value_num)

}





#[function_component(MyInput)]
pub fn my_input(props: &Props) -> Html {
    
    let state = {
        let value_num = props.value;
        let valid_range = props.valid_range;
        use_state_eq( move || {
            // StateData::Valid(value_num)
            validate_num(valid_range, value_num)
        })
    };

    let onchange = {
    
        let state = state.clone();
        let valid_range = props.valid_range;

        let handle_onchange = props.handle_onchange.clone();
        let id = props.id;

        Callback::from(move |event: Event| {

            let value_string = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            let new_state = validate_str(valid_range, &value_string);

            state.set(new_state.clone());

            match new_state {
                StateData::Valid(new_value) => {
                    handle_onchange.emit(ValueUpdateData{new_value, param_id: id});
                },
                StateData::Invalid(_) => {},
                StateData::None => {},
            };

        })
    };

    // {
    //     let value_num = props.value;
    //     let valid_range = props.valid_range.clone();
    //     let handle_onchange = props.handle_onchange.clone();
    //     let id = props.id;
    //     use_effect( move || {
    //         match validate_num(valid_range, value_num) {
    //             StateData::Valid(new_value) => {
    //                 handle_onchange.emit(ValueUpdateData{new_value, param_id: id});
    //             },
    //             StateData::Invalid(_v) => {},
    //             StateData::None => {},
    //         };
    //     })
    // };




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
                        StateData::None => false,
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
                        StateData::None => "my-invalid-input",
                    } 
                } 
                value={
                    match &*state { 
                        StateData::Valid(v) => v.to_string().clone(),
                        StateData::Invalid(v) => v.to_string().clone(),
                        StateData::None => "".to_string(),
                    } 
                }
                aria-describedby={aria_id.clone()} 
                onchange={onchange} 
            />

            <span id={aria_id} class="hidden">{&props.description}</span>

        </div>
    }

}