use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ModalProps {
    #[prop_or_default]
    pub title: String,
    pub is_visible: bool,
    pub onclose: Callback<()>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let onclose = {
        let parent_onclose = props.onclose.clone();
        Callback::from(move |_ev: MouseEvent| {
            parent_onclose.emit(());
        })
    };

    if props.is_visible {
        html! {<>


            // // <!-- Modal toggle -->
            // <button
            //     data-modal-target="select-modal"
            //     data-modal-toggle="select-modal"
            //     class="block text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
            //     type="button"
            // >
            // {"Toggle modal"}
            // </button>



            // <!-- Main modal -->
            <div
                id="select-modal"
                tabindex="-1"
                aria-hidden="true"
                // class="hidden overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full"
                // class="flex backdrop-blur-sm overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full"

                class={classes!(vec![
                    // "z-[100] fixed top-0 left-0 right-0 w-full overflow-x-hidden overflow-y-auto h-full max-h-full",
                    "overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 w-full md:inset-0 h-[calc(100%-1rem)] max-h-full",
                    // "flex flex-col items-center justify-center backdrop-blur-sm"
                    // "flex flex-col items-center justify-center backdrop-brightness-50 bg-white/30"
                    "flex flex-col items-center justify-center backdrop-opacity-40 backdrop-invert"
                ])}

            >
                <div class="relative p-4 w-full max-w-md max-h-full">

                    // <!-- Modal content -->
                    <div class="relative bg-white rounded-lg shadow dark:bg-gray-700">

                        // <!-- Modal header -->
                        <div class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-600">
                            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                                {&props.title}
                            </h3>
                            <button
                                type="button"
                                class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm h-8 w-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white"
                                // data-modal-toggle="select-modal"
                                onclick={onclose}
                            >
                                <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
                                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
                                </svg>
                                <span class="sr-only">
                                    {"Close modal"}
                                </span>
                            </button>
                        </div>

                        // <!-- Modal body -->
                        <div class="p-4 md:p-5">


                            { for props.children.iter() }




                            // <p class="text-gray-500 dark:text-gray-400 mb-4">{"Select your desired position:"}</p>

                            // <ul class="space-y-4 mb-4">
                            //     <li>
                            //         <input type="radio" id="job-1" name="job" value="job-1" class="hidden peer" required=true />
                            //         <label for="job-1" class="inline-flex items-center justify-between w-full p-5 text-gray-900 bg-white border border-gray-200 rounded-lg cursor-pointer dark:hover:text-gray-300 dark:border-gray-500 dark:peer-checked:text-blue-500 peer-checked:border-blue-600 peer-checked:text-blue-600 hover:text-gray-900 hover:bg-gray-100 dark:text-white dark:bg-gray-600 dark:hover:bg-gray-500">
                            //             <div class="block">
                            //                 <div class="w-full text-lg font-semibold">{"UI/UX Engineer"}</div>
                            //                 <div class="w-full text-gray-500 dark:text-gray-400">{"Flowbite"}</div>
                            //             </div>
                            //             <svg class="w-4 h-4 ms-3 rtl:rotate-180 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 10"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 5h12m0 0L9 1m4 4L9 9"/></svg>
                            //         </label>
                            //     </li>
                            //     <li>
                            //         <input type="radio" id="job-2" name="job" value="job-2" class="hidden peer" />
                            //         <label for="job-2" class="inline-flex items-center justify-between w-full p-5 text-gray-900 bg-white border border-gray-200 rounded-lg cursor-pointer dark:hover:text-gray-300 dark:border-gray-500 dark:peer-checked:text-blue-500 peer-checked:border-blue-600 peer-checked:text-blue-600 hover:text-gray-900 hover:bg-gray-100 dark:text-white dark:bg-gray-600 dark:hover:bg-gray-500">
                            //             <div class="block">
                            //                 <div class="w-full text-lg font-semibold">{"React Developer"}</div>
                            //                 <div class="w-full text-gray-500 dark:text-gray-400">{"Alphabet"}</div>
                            //             </div>
                            //             <svg class="w-4 h-4 ms-3 rtl:rotate-180 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 10"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 5h12m0 0L9 1m4 4L9 9"/></svg>
                            //         </label>
                            //     </li>
                            // </ul>
                            // <button class="text-white inline-flex w-full justify-center bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                            //     {"Next step"}
                            // </button>




                        </div>
                    </div>
                </div>
            </div>




        </>}
    } else {
        html! {}
    }
}
