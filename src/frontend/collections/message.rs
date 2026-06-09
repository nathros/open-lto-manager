use dioxus::prelude::*;

use crate::frontend::level::Level;

#[derive(Clone, PartialEq, Eq)]
pub struct MessageDetails {
    pub level: Level,
    pub text: String,
}

impl Default for MessageDetails {
    fn default() -> Self {
        Self {
            level: Level::Error,
            text: "".to_string(),
        }
    }
}

#[component]
pub fn Message(details: MessageDetails) -> Element {
    rsx! {
        if !details.text.is_empty() {
            div { class: format!("flex-row message {}", details.level.to_class()),
                div { class: format!("icon bg {}", details.level.to_class()) }
                span { "{details.text}" }
            }
        }
    }
}
