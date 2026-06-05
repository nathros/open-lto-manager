use dioxus::prelude::*;

use crate::frontend::level::Level;

#[component]
pub fn Message(level: Level, text: String) -> Element {
    rsx! {
        if !text.is_empty() {
            div { class: format!("flex-row message {}", level.to_class()),
                div { class: format!("icon bg {}", level.to_class()) }
                span { "{text}" }
            }
        }
    }
}
