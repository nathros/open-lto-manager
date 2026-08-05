use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn systemCanvas(); // Defined in assets/js/header.js
    fn showModal(id: &str); // Defined in assets/js/common.js
    fn hidePopover(id: &str); // Defined in assets/js/common.js
    fn copyToClipboard(str: &str); // Defined in assets/js/common.js
    fn downloadFile(url: &str, method: &str, contentType: &str, ext: &str, body: &str); // Defined in assets/js/common.js
}

pub fn js_system_canvas() {
    systemCanvas();
}

pub fn js_show_modal(id: &str) {
    showModal(id);
}

pub fn js_hide_popover(id: &str) {
    hidePopover(id);
}

pub fn js_copy_to_clipboard(str: &str) {
    copyToClipboard(str);
}

pub fn js_download_file(url: &str, ext: &str, body: Value) {
    downloadFile(
        url,
        "POST",
        "application/json",
        ext,
        body.to_string().as_str(),
    );
}
