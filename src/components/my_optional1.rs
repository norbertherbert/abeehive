use regex::Regex;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::my_label::MyLabel;

use crate::params::param_values::{ParamValue, ValueUpdateData};
use crate::prm::typ::PrmVal;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: u8,
    pub label: &'static str,
    pub description: &'static str,
    pub value: ParamValue,
    pub disabled_value: PrmVal,
    pub default_value: PrmVal,
    pub valid_range: (PrmVal, PrmVal),
    pub handle_onchange: Callback<ValueUpdateData>,
}

const REGEX_STR: &str = "^[0-9]+$";
// const REGEX_STR: &str = "^[A-Fa-f0-9]{8}$";
const RADIX_NUM: u32 = 10;
// const RADIX_NUM: u32 = 16;
pub fn format_num(value_num: PrmVal) -> String {
    format!("{}", value_num)
    // format!("{:08x}", value_num)
}

pub fn validate_num(valid_range: (PrmVal, PrmVal), value_num: PrmVal) -> ParamValue {
    if value_num < valid_range.0 || valid_range.1 < value_num {
        ParamValue::Invalid(format_num(value_num))
    } else {
        ParamValue::Valid(value_num)
    }
}

pub fn validate_str(valid_range: (PrmVal, PrmVal), value_str: &str) -> ParamValue {
    if value_str == "" {
        return ParamValue::None;
    }

    let re = Regex::new(REGEX_STR).unwrap();
    if !re.is_match(value_str) {
        return ParamValue::Invalid(value_str.to_string());
    }

    let Ok(value_num) = PrmVal::from_str_radix(value_str, RADIX_NUM) else {
        return ParamValue::Invalid(value_str.to_string());
    };

    validate_num(valid_range, value_num)
}

#[function_component(MyOptionalInput)]
pub fn my_optional_input(props: &Props) -> Html {
    let checkbox_noderef = use_node_ref();
    let text_input_noderef = use_node_ref();

    let on_toggle = {
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

                handle_onchange.emit(ValueUpdateData {
                    new_param_value: ParamValue::Valid(default_value),
                    param_id: id,
                });
            } else {
                // text_input.set_value(&disabled_value.to_string());
                // text_input.set_disabled(true);
                // text_input.set_class_name(&disabled_classes);

                handle_onchange.emit(ValueUpdateData {
                    new_param_value: ParamValue::Valid(disabled_value),
                    param_id: id,
                });
            }
        })
    };

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

            handle_onchange.emit(ValueUpdateData {
                new_param_value,
                param_id: id,
            });
        })
    };

    let param_value = props.value.clone();
    let aria_id = format!("{}-aria", &props.id);
    let checkbox_id = format!("{}-checkbox", &props.id);

    let is_disabled = props.value == ParamValue::Valid(props.disabled_value);

    html! {
        <div>

            <MyLabel
                input_element_id={props.id}
                label={props.label}
                description={props.description}
                is_valid= {
                    match &param_value {
                        ParamValue::Valid(_) => true,
                        _ => false,
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
                            checked={ !is_disabled }
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
                        match is_disabled {
                            true => {
                                classes!("ps-14", "my-disabled-optional-input")
                            },
                            false => {
                                match &param_value {
                                    ParamValue::Valid(_) => classes!("ps-14", "my-valid-optional-input"),
                                    _ => classes!("ps-14", "my-invalid-optional-input"),
                                }
                            },
                        }
                    }
                    value={
                        match is_disabled {
                            true => {
                                props.disabled_value.to_string().clone()
                            },
                            false => {
                                match &param_value {
                                    ParamValue::Valid(v) => v.to_string().clone(),
                                    ParamValue::Invalid(v) => v.to_string().clone(),
                                    ParamValue::None => "".to_string(),
                                }
                            },
                        }
                    }
                    disabled={ is_disabled }
                    aria-describedby={aria_id.clone()}
                    onchange={onchange}
                />

                // Aria
                <span id={aria_id} class="hidden">{&props.description}</span>

            </div>

        </div>
    }
}
