use dioxus::prelude::*;

use crate::shared::level::{Level, level_style};

#[component]
pub fn Message(level: Level, text: String) -> Element {
    rsx! {
        if !text.is_empty() {
            p { style: level_style(level), "{text}" }
        }
    }
}
