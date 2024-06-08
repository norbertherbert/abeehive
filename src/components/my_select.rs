use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::my_label::MyLabel;

use crate::prm::{
    typ::DistinctVal,
    val::PrmVVal,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: u8,
    pub label: &'static str,
    pub description: &'static str,
    pub select_options: &'static [DistinctVal],

    pub vval: PrmVVal,
    pub handle_onchange: Callback<(u8, String)>,
}

#[function_component(MySelect)]
pub fn my_select(props: &Props) -> Html {

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

    let value = match props.vval {
        PrmVVal::Valid(v) => v.to_string(),
        _ => "".to_string(), // TODO: select the right default value!
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

            <div>
                <select
                    id = { props.id.to_string() }
                    // class = "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    class = {
                        match &vval {
                            PrmVVal::Valid(_) => "my-valid-input",
                            PrmVVal::Invalid(_) => "my-invalid-input",
                            PrmVVal::InvalidTxt(_) => "my-invalid-input",
                        }
                    }

                    value = { value.to_string() }

                    aria-describedby = { aria_id.clone() }
                    onchange = { onchange }
                >

                    // <option value = { "0xaaa" }>{ "invalid value for testing" }</option>

                    {
                        props.select_options.iter().map(|item| {
                            html!{ 
                                <option value = { item.val.to_string() } selected = { item.val.to_string() == value } >
                                    { format!("{} - {}", item.val, item.txt) }
                                </option> 
                            }
                        }).collect::<Html>()
                    }

                    {
                        match vval {
                            PrmVVal::Valid(_) => html!(),
                            PrmVVal::Invalid((val, _)) => {
                                html! (
                                    <option value = { "" } selected = true>
                                        { format!("{} - Please select a valid value!", val) }
                                    </option>
                                )
                            },
                            PrmVVal::InvalidTxt((_, _)) => {
                                html! (
                                    <option value = { "" } selected = true>
                                        { "Please select a valid value!" }
                                    </option>
                                )
                            }
                        }

                    }

                </select>

                <span id = { aria_id } class = "hidden">{ &props.description }</span>
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
