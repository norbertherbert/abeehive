use wasm_bindgen::prelude::*;

#[wasm_bindgen(module="/assets/flowbite.js")]
extern "C" {
    pub fn initFlowbite();
}
