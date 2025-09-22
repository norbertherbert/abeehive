use yew::prelude::*;

use crate::templates::CfgTemplate;

#[derive(Debug, PartialEq)]
pub enum NavbarAction {
    New(CfgTemplate),
    Close,
    GetFromFile,
    SaveToFile,
    AboutModalShow
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub source_name: String,
    pub onclick: Callback<NavbarAction>,
}

#[function_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    
    let onclick_new_tracking_gps = {
        let parent_onclick = props.onclick.clone();
        Callback::from(move |_ev: MouseEvent| {
            parent_onclick.emit(NavbarAction::New(CfgTemplate::TrackingGps));
        })
    };

    let onclick_new_tracking_wifi_gps = {
        let parent_onclick = props.onclick.clone();
        Callback::from(move |_ev: MouseEvent| {
            parent_onclick.emit(NavbarAction::New(CfgTemplate::TrackingWifiGps));
        })
    };

    let onclick_new_tracking_ble_gps = {
        let parent_onclick = props.onclick.clone();
        Callback::from(move |_ev: MouseEvent| {
            parent_onclick.emit(NavbarAction::New(CfgTemplate::TrackingBleGps));
        })
    };

    let onclick_new_tracking_ble_gps_realtime = {
        let parent_onclick = props.onclick.clone();
        Callback::from(move |_ev: MouseEvent| {
            parent_onclick.emit(NavbarAction::New(CfgTemplate::TrackingBleGpsRealtime));
        })
    };

    let onclick_new_ble_geozoning = {
        let parent_onclick = props.onclick.clone();
        Callback::from(move |_ev: MouseEvent| {
            parent_onclick.emit(NavbarAction::New(CfgTemplate::BleGeozoning));
        })
    };

    let onclick_new_ble_scan_collection = {
        let parent_onclick = props.onclick.clone();
        Callback::from(move |_ev: MouseEvent| {
            parent_onclick.emit(NavbarAction::New(CfgTemplate::BleScanCollection));
        })
    };

    let onclick_new_angle_detection = {
        let parent_onclick = props.onclick.clone();
        Callback::from(move |_ev: MouseEvent| {
            parent_onclick.emit(NavbarAction::New(CfgTemplate::AngleDetection));
        })
    };

    let onclick_new_ble_beaconing = {
        let parent_onclick = props.onclick.clone();
        Callback::from(move |_ev: MouseEvent| {
            parent_onclick.emit(NavbarAction::New(CfgTemplate::BleBeaconing));
        })
    };



    let onclick_close = {
        let parent_onclick = props.onclick.clone();
        Callback::from(move |_ev: MouseEvent| {
            parent_onclick.emit(NavbarAction::Close);
        })
    };
    let onclick_get_from_file = {
        let parent_onclick = props.onclick.clone();
        Callback::from(move |_ev: MouseEvent| {
            parent_onclick.emit(NavbarAction::GetFromFile);
        })
    };
    let onclick_save_to_file = {
        let parent_onclick = props.onclick.clone();
        Callback::from(move |_ev: MouseEvent| {
            parent_onclick.emit(NavbarAction::SaveToFile);
        })
    };

    let onclick_about_modal_toggle = {
        let parent_onclick = props.onclick.clone();
        Callback::from(move |_ev: MouseEvent| {
            parent_onclick.emit(NavbarAction::AboutModalShow);
        })
    };

    html! {<>

        <nav class="bg-white dark:bg-gray-900 fixed w-full z-20 top-0 start-0 border-b border-gray-200 dark:border-gray-600">
            <div class="w-full flex flex-wrap items-center justify-start gap-2 mx-auto p-4">




                // <a href="https://abeeway.com/" target="_blank" class="flex items-center space-x-3 rtl:space-x-reverse">
                <a href="https://abeeway.com/" target="_blank">
                    <img 
                        src="assets/beequeen.png" 
                        class="logo mr-3 h-8 sm:h-12" 
                        alt="Beequeen Logo" 
                    />
                </a>


                <div class="flex items-end gap-3">

                    <a 
                        href="https://abeeway.com/" target="_blank" 
                        // class = "hidden sm:block"
                    >
                        <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">
                            {"BeeQueen - "}<i>{"for AT2 Fw v2.6"}</i>
                        </span>
                    </a>

                    <div class="self-center pt-1 pl-10 font-mono whitespace-nowrap dark:text-white" >
                        { props.source_name.clone() }
                    </div>

                </div>





                // <div class="flex md:order-2 space-x-3 md:space-x-0 rtl:space-x-reverse">
                    // <button
                    //     type="button"
                    //     class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                    // >
                    //     {"Get started"}
                    // </button>
                    <button
                        data-collapse-toggle="navbar-sticky"
                        type="button"
                        class="ml-auto inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
                        aria-controls="navbar-sticky"
                        aria-expanded="false"
                    >
                        <span class="sr-only">
                            {"Open main menu"}
                        </span>
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 17 14">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h15M1 7h15M1 13h15"/>
                        </svg>
                    </button>
                // </div>





                // The block of all Menus
                <div class="ml-auto items-center justify-between hidden w-full md:flex md:w-auto md:order-1" id="navbar-sticky">
                // <div class="hidden w-full md:block md:w-auto" id="navbar-sticky">
                    <ul class="flex flex-col p-4 md:p-0 mt-4 font-medium border border-gray-100 rounded-lg bg-gray-50 md:space-x-8 rtl:space-x-reverse md:flex-row md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">


                        // "File" menu
                        <li>
                            <button
                                id="dropdownNavbarLinkFile"
                                data-dropdown-toggle="dropdownNavbarFile"
                                // data-dropdown-trigger="hover"
                                // data-dropdown-delay="500"
                                class="flex items-center justify-between w-full py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 md:w-auto dark:text-white md:dark:hover:text-blue-500 dark:focus:text-white dark:border-gray-700 dark:hover:bg-gray-700 md:dark:hover:bg-transparent"
                            >
                                {"File"}
                                <svg class="w-2.5 h-2.5 ms-2.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                                </svg>
                            </button>
                            // <!-- Dropdown menu -->
                            <div id="dropdownNavbarFile" class="z-10 hidden font-normal bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700 dark:divide-gray-600">
                                <ul class="py-2 text-sm text-gray-700 dark:text-gray-400" aria-labelledby="dropdownLargeButton">
                                <li aria-labelledby="dropdownNavbarLink">

                                        <button 
                                            id="configTemplatesButton" 
                                            data-dropdown-toggle="configTemplatesDropdown" 
                                            data-dropdown-placement="right-start" 
                                            type="button" 
                                            class="flex items-center justify-between w-full px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
                                        >    
                                            { "New" }
                                            <svg class="w-2.5 h-2.5 ms-3 rtl:rotate-180" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 6 10">
                                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 9 4-4-4-4"/>
                                            </svg>
                                        </button>
                                        
                                        // Templates Submenu
                                        <div id="configTemplatesDropdown" class="z-10 hidden bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700">
                                            <ul class="py-2 text-sm text-gray-700 dark:text-gray-200" aria-labelledby="doubleDropdownButton">
                                                <li>
                                                    <a href="#" 
                                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                                        onclick = { onclick_new_tracking_gps.clone() }
                                                    >
                                                        { "Tracking GPS" }
                                                    </a>
                                                </li>
                                                <li>
                                                    <a href="#" 
                                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                                        onclick = { onclick_new_tracking_wifi_gps.clone() }
                                                    >
                                                        { "Tracking WiFi+GPS" }
                                                    </a>
                                                </li>
                                                <li>
                                                    <a href="#" 
                                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                                        onclick = { onclick_new_tracking_ble_gps.clone() }
                                                    >
                                                        { "Tracking BLE+GPS" }
                                                    </a>
                                                </li>
                                                <li>
                                                    <a href="#" 
                                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                                        onclick = { onclick_new_tracking_ble_gps_realtime.clone() }
                                                    >
                                                        { "Tracking BLE+GPS Realtime-15s" }
                                                    </a>
                                                </li>
                                                <li>
                                                    <a href="#" 
                                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                                        onclick = { onclick_new_ble_geozoning.clone() }
                                                    >
                                                        { "BLE Geofencing" }
                                                    </a>
                                                </li>
                                                <li>
                                                    <a href="#" 
                                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                                        onclick = { onclick_new_ble_scan_collection.clone() }
                                                    >
                                                        { "Scan Collection" }
                                                    </a>
                                                </li>
                                                <li>
                                                    <a href="#" 
                                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                                        onclick = { onclick_new_angle_detection.clone() }
                                                    >
                                                        { "Angle Detection" }
                                                    </a>
                                                </li>
                                                <li>
                                                    <a href="#" 
                                                        class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white"
                                                        onclick = { onclick_new_ble_beaconing.clone() }
                                                    >
                                                        { "BLE Beaconing" }
                                                    </a>
                                                </li>
                                            </ul>
                                        </div>
                                    </li>
                                    <li>
                                        <a href="#"
                                            class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
                                            onclick = { onclick_get_from_file.clone() }
                                        >
                                            {"Open..."}
                                        </a>
                                    </li>
                                    <li>
                                        <a href="#"
                                            class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
                                            onclick = { onclick_save_to_file.clone() }
                                        >
                                            {"Save as..."}
                                        </a>
                                    </li>
                                    <li>
                                        <a href="#"
                                            class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
                                            onclick = { onclick_close.clone() }
                                        >
                                            {"Close"}
                                        </a>
                                    </li>
                                </ul>
                            </div>
                        </li>

                        // "Help" menu
                        <li>
                            <button
                                id="dropdownNavbarLinkHelp"
                                data-dropdown-toggle="dropdownNavbarHelp"
                                // data-dropdown-trigger="hover"
                                // data-dropdown-delay="500"
                                class="flex items-center justify-between w-full py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 md:w-auto dark:text-white md:dark:hover:text-blue-500 dark:focus:text-white dark:border-gray-700 dark:hover:bg-gray-700 md:dark:hover:bg-transparent"
                            >
                                {"Help"}
                                <svg class="w-2.5 h-2.5 ms-2.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
                                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
                                </svg>
                            </button>
                            // <!-- Dropdown menu -->
                            <div id="dropdownNavbarHelp" class="z-10 hidden font-normal bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700 dark:divide-gray-600">
                                <ul class="py-2 text-sm text-gray-700 dark:text-gray-400" aria-labelledby="dropdownLargeButton">
                                    <li>
                                        <a 
                                            href="https://docs.thingpark.com/thingpark-location/firmware/AT2_v2.6/introduction/"
                                            target="_blank"
                                            class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
                                            // onclick = { onclick_get_from_usb.clone() }
                                        >
                                            {"User guide..."}
                                        </a>
                                    </li>
                                    <li>
                                        <a href="#"
                                            class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white"
                                            onclick = { onclick_about_modal_toggle.clone() }
                                        >
                                            {"About..."}
                                        </a>
                                    </li>
                                </ul>
                            </div>
                        </li>

                        <li>

                            <a href="https://nano-things.net/beehive/" target="_blank">
                                <img 
                                    src="assets/beehive.png" 
                                    class="logo inline-block h-6 pr-1 pb-1" 
                                    alt="BeeHive Logo" 
                                />
                                {"BeeHive"}
                            </a>  

                        </li>

                    </ul>
                </div>
            </div>
        </nav>

    </>}
}
