use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/assets/file_api.js")]
extern "C" {
    pub fn open_config_file_str() -> js_sys::Promise;
    pub fn save_config_file_str(file_name: &str, config_str: &str) -> js_sys::Promise;
}
