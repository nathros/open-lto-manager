use dioxus::prelude::*;

use crate::shared::level::Level;

#[component]
pub fn Message(level: Level, text: String) -> Element {
    rsx! {
        if !text.is_empty() {
            p { style: level.to_style(), "{text}" }
        }
    }
}
