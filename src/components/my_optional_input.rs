use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::{
    RadixDisp,
    my_label::MyLabel
};
use crate::prm::{
    typ::PrmVal, 
    val::PrmVVal,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: u8,
    pub label: &'static str,
    pub description: &'static str,
    pub disabled_value: PrmVal,
    pub default_value: PrmVal,
    pub radix_disp: RadixDisp,
    pub vval: PrmVVal,
    pub handle_onchange: Callback<(u8, String)>,
}

#[function_component(MyOptionalInput)]
pub fn my_optional_input(props: &Props) -> Html {

    // let checkbox_noderef = use_node_ref();
    // let text_input_noderef = use_node_ref();

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
            if checked {
                handle_onchange.emit((
                    id,
                    default_value.to_string()
                ));
            } else {
                handle_onchange.emit((
                    id,
                    disabled_value.to_string(),
                ));
            }
        })
    };

    let onchange = {
        let handle_onchange = props.handle_onchange.clone();
        let id = props.id;
        Callback::from(move |event: Event| {
            let txt = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            handle_onchange.emit((id, txt));
        })
    };


    let vval = props.vval.clone();
    let aria_id = format!("{}-aria", &props.id);
    let checkbox_id = format!("{}-checkbox", &props.id);

    let is_disabled = match vval {
        PrmVVal::Valid(current_valid_value) => {
            current_valid_value == props.disabled_value
        } 
        _ => false
    };

    html! {
        <div>

            <MyLabel
                input_element_id = { props.id }
                label = { props.label }
                description = { props.description}
                is_valid = {
                    match &vval {
                        PrmVVal::Valid(_) => true,
                        PrmVVal::Invalid(_) => false,
                        PrmVVal::InvalidTxt(_) => false,
                    }
                }
            />

            // Checkbox + Input field
            <div class="relative">

                // Checkbox (on top of the input field)
                <div class="absolute inset-y-0 start-0 flex items-center ps-2.5">
                    <label class="inline-flex items-center cursor-pointer">
                        <input
                            type = "checkbox"
                            id = {checkbox_id}
                            // ref = {checkbox_noderef}
                            class = "sr-only peer"
                            checked = { !is_disabled }
                            onchange = {on_toggle}
                        />
                        <div
                            class = "relative w-9 h-5 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"
                        ></div>
                    </label>
                </div>

                // Input field
                <input
                    type = "text"
                    autocomplete = "off"
                    id = {props.id.to_string()}
                    // ref = {text_input_noderef}
                    class = {
                        match is_disabled {
                            true => {
                                classes!("ps-14", "my-disabled-optional-input")
                            },
                            false => {
                                match &vval {
                                    PrmVVal::Valid(_) => classes!("ps-14", "my-valid-optional-input"),
                                    PrmVVal::Invalid(_) => classes!("ps-14", "my-invalid-optional-input"),
                                    PrmVVal::InvalidTxt(_) => classes!("ps-14", "my-invalid-optional-input"),
                                }
                            },
                        }
                    }
                    value = {
                        match is_disabled {
                            true => {
                                props.disabled_value.to_string().clone()
                            },
                            false => {
                                match &vval {
                                    PrmVVal::Valid(val) => match props.radix_disp {
                                        RadixDisp::Dec => format!("{}", val),
                                        RadixDisp::Hex => format!("0x{:08x}", val),
                                    },
                                    PrmVVal::Invalid((val, _)) => match props.radix_disp {
                                        RadixDisp::Dec => format!("{}", val),
                                        RadixDisp::Hex => format!("0x{:08x}", val),
                                    },
                                    PrmVVal::InvalidTxt((txt, _)) => txt.clone(),
                                }
                            },
                        }
                    }
                    disabled = { is_disabled }
                    aria-describedby = { aria_id.clone() }
                    onchange = { onchange }
                />

                // Aria
                <span id={aria_id} class="hidden">{&props.description}</span>

            </div>

            // <span id={aria_id} class = "hidden">
            {
                match &vval {
                    PrmVVal::Valid(_) => html!(),
                    PrmVVal::Invalid((_, err)) => html!( <span class = { "my-error-msg" }>{ err.clone() }</span> ),
                    PrmVVal::InvalidTxt((_, err)) => html!( <span class = { "my-error-msg" }>{ err.clone() }</span> ),
                }
            }
            // </span>

        </div>
    }
}
