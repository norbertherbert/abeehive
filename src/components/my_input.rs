use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::{
    RadixDisp,
    my_label::MyLabel
};

use crate::prm::val::PrmVVal;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: u8,
    pub label: &'static str,
    pub description: &'static str,
    pub radix_disp: RadixDisp,
    pub vval: PrmVVal,
    pub handle_onchange: Callback<(u8, String)>,
}

#[function_component(MyInput)]
pub fn my_input(props: &Props) -> Html {

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
                <input
                    type="text"
                    autocomplete = "off"
                    id={props.id.to_string()}
                    class = {
                        match &vval {
                            PrmVVal::Valid(_) => "my-valid-input",
                            PrmVVal::Invalid(_) => "my-invalid-input",
                            PrmVVal::InvalidTxt(_) => "my-invalid-input",
                        }
                    }
                    value = {
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
                    }
                    aria-describedby={aria_id.clone()}
                    onchange={onchange}
                />

                // Aria
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
