use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn systemCanvas(); // Defined in assets/js/header.js
}

pub fn js_system_canvas() {
    systemCanvas();
}
