use dioxus::html::{InteractionLocation, MouseEvent};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn systemCanvas(); // Defined in assets/js/header.js
    fn showModal(id: &str); // Defined in assets/js/common.js
    fn hidePopover(id: &str); // Defined in assets/js/common.js
    fn copyToClipboard(str: &str); // Defined in assets/js/common.js
    fn downloadFile(url: &str, method: &str, contentType: &str, body: &str); // Defined in assets/js/common.js
    fn showContextMenu(id: &str, xPos: i32, yPos: i32); // Defined in assets/js/common.js
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

pub fn js_download_file(url: &str, body: Value) {
    downloadFile(url, "POST", "application/json", body.to_string().as_str());
}

pub fn js_show_context_menu(id: &str, event: &MouseEvent) {
    let pos = event.data.page_coordinates();
    showContextMenu(id, pos.x as i32, pos.y as i32);
}
