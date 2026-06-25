use wasm_bindgen::prelude::*;

use crate::shared::models::database::user::model_user::RecordUserConfig;

#[wasm_bindgen]
extern "C" {
    fn systemCanvas(); // Defined in assets/js/header.js
    fn setGlobalStyle(o: &str, l: &str); // Defined in assets/js/common.js
}

pub fn js_system_canvas() {
    systemCanvas();
}

pub fn js_global_style(user: &RecordUserConfig) {
    #[cfg(feature = "web")]
    setGlobalStyle(
        &user.accent_colour,
        format!("{:?}", user.icon_theme).to_lowercase().as_str(),
    );
}
