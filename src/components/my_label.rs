use yew::prelude::*;
// use std::fmt;
// use gloo::console::log;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_element_id: u8,
    pub label: &'static str,
    pub description: &'static str,
    pub is_valid: bool,
}

#[function_component(MyLabel)]
pub fn my_label(props: &Props) -> Html {
    let tooltip_id = format!("{}-tooltip", props.input_element_id);

    html! {
        <div>

            // Input field label + ? icon
            <div class="flex justify-start items-center">
                <label
                    for={props.input_element_id.to_string()}
                    class={
                        if props.is_valid {
                            "my-valid-label"
                        } else {
                            "my-invalid-label"
                        }
                    }
                >
                    {&props.label}
                    <span class="ml-1 text-xs italic">
                        {"- "}{props.input_element_id.to_string()}
                    </span>
                </label>

                // ? icon
                <svg
                    data-tooltip-target={tooltip_id.clone()}
                    data-bs-toggle="tooltip"
                    data-bs-title={props.description}
                    data-bs-delay={"{ \"show\": 500, \"hide\": 0 }"}
                    class="ml-1 mb-1 w-4 h-4 text-gray-800 dark:text-white"
                    aria-hidden="true"
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    fill="none"
                    viewBox="0 0 24 24">
                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 11h2v5m-2 0h4m-2.592-8.5h.01M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"/>
                </svg>

                // Tooltip
                <div
                    id={tooltip_id}
                    role="tooltip"
                    class="absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-white transition-opacity duration-300 bg-gray-900 rounded-lg shadow-sm opacity-75 tooltip dark:bg-gray-700"
                >
                    {&props.description}
                    <div class="tooltip-arrow" data-popper-arrow="true"></div>
                </div>
            </div>

        </div>
    }
}
