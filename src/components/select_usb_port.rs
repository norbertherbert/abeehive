use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub select_options: Vec<String>,
    pub value: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(SelectUsbPort)]
pub fn select_usb_port(props: &Props) -> Html {
    let onchange = {
        let handle_onchange = props.handle_onchange.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            handle_onchange.emit(value);
        })
    };

    let aria_id = format!("{}-aria", props.id);

    html! {
        <div>

            <label
                for={props.id}
                class={ "my-valid-label" }
            >
                {props.label}
            </label>

            <select
                id={props.id}

                value={props.value.clone()}

                onchange={onchange}
                aria-describedby={aria_id.clone()}
                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
            >

                <option
                    value=""
                    selected={"" == &props.value}
                >
                    {"-"}
                </option>

                {
                    props.select_options.iter().map(|item| {
                        html!{ <option value={item.clone()} selected={item == &props.value} >{item.clone()}</option> }
                    }).collect::<Html>()
                }

            </select>

            <span id={aria_id} class="hidden">{props.description}</span>

        </div>
    }
}
