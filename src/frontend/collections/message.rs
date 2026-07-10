use dioxus::prelude::*;

use crate::{
    either,
    frontend::{css::Css, level::Level},
};

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
pub fn Message(
    details: MessageDetails,
    #[props(optional, default = false)] small: bool,
) -> Element {
    rsx! {
        if !details.text.is_empty() {
            div {
                class: [
                    Css::FLEX_ROW,
                    Css::FLEX_ALIGN_CENTRE,
                    Css::MESSAGE,
                    either!(small, Css::SM, ""),
                    details.level.to_class(),
                ]
                    .concat(),
                div { class: [Css::ICON, Css::BG, details.level.to_class()].concat() }
                span { "{details.text}" }
            }
        }
    }
}
