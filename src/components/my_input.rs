use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use regex::Regex;

use crate::components::my_label::MyLabel;
use crate::params::{
    ParamType,
    param_id::ParamId,
    param_values::{ParamValue, ValueUpdateData},
};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: ParamId,
    pub label: &'static str,
    pub description: &'static str,
    pub value: ParamValue,
    pub valid_range: (ParamType, ParamType),
    pub handle_onchange: Callback<ValueUpdateData>,
}

const REGEX_STR: &str = "^[0-9]+$";
// const REGEX_STR: &str = "^[A-Fa-f0-9]{8}$";
const RADIX_NUM: u32 = 10;
// const RADIX_NUM: u32 = 16;
pub fn format_num(value_num: ParamType) -> String {
    format!("{}", value_num)
    // format!("{:08x}", value_num)
}

pub fn validate_num(valid_range: (ParamType, ParamType), value_num: ParamType) -> ParamValue {
    if value_num < valid_range.0 || valid_range.1 < value_num {
        ParamValue::Invalid(format_num(value_num))
    } else {
        ParamValue::Valid(value_num)
    }
}

pub fn validate_str(valid_range: (ParamType, ParamType), value_str: &str) -> ParamValue {

    if value_str=="" {
        return ParamValue::None;
    } 

    let re = Regex::new(REGEX_STR).unwrap();
    if !re.is_match(value_str) {
        return ParamValue::Invalid(value_str.to_string());
    } 

    let Ok(value_num) = ParamType::from_str_radix(value_str, RADIX_NUM) else {
        return ParamValue::Invalid(value_str.to_string());
    };

    validate_num(valid_range, value_num)

}

#[function_component(MyInput)]
pub fn my_input(props: &Props) -> Html {
 
    let onchange = {
    
        let valid_range = props.valid_range;
        let handle_onchange = props.handle_onchange.clone();
        let id = props.id;

        Callback::from(move |event: Event| {

            let value_string = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            let new_param_value = validate_str(valid_range, &value_string);

            handle_onchange.emit(ValueUpdateData{new_param_value, param_id: id});

        })
    };

    // {
    //     let valid_range = props.valid_range;
    //     let handle_onchange = props.handle_onchange.clone();
    //     let id = props.id;
    //     let param_value = props.value.clone();

    //     use_effect_with(param_value.clone(), move |_| {

    //         let calculated_param_value = match param_value.clone() {
    //             ParamValue::Valid(value_num) => {
    //                 validate_num(valid_range, value_num)
    //             },
    //             ParamValue::Invalid(value_string) => {
    //                 validate_str(valid_range, &value_string)
    //             },
    //             ParamValue::None => {
    //                 ParamValue::None
    //             },
    //         };

    //         if param_value != calculated_param_value {
    //             handle_onchange.emit(ValueUpdateData{new_param_value: calculated_param_value, param_id: id});
    //         }

    //     })
    // }

    let param_value = props.value.clone();
    let aria_id = format!("{}-aria", &props.id);

    html! {
        <div>
            <MyLabel
                input_element_id={props.id}
                label={props.label}
                description={props.description}
                is_valid= { 
                    match &param_value { 
                        ParamValue::Valid(_) => true,
                        ParamValue::Invalid(_) => false,
                        ParamValue::None => false,
                    } 
                }
            />

            <input 
                type="text"
                autocomplete="off"
                id={props.id.to_string()}
                class={ 
                    match &param_value {
                        ParamValue::Valid(_) => "my-valid-input",
                        ParamValue::Invalid(_) => "my-invalid-input",
                        ParamValue::None => "my-invalid-input",
                    } 
                } 
                value={
                    match &param_value { 
                        ParamValue::Valid(v) => format_num(*v),
                        ParamValue::Invalid(v) => v.to_string().clone(),
                        ParamValue::None => "".to_string(),
                    } 
                }
                aria-describedby={aria_id.clone()} 
                onchange={onchange} 
            />

            <span id={aria_id} class="hidden">{&props.description}</span>

        </div>
    }

}