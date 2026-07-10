use dioxus::prelude::*;

use crate::frontend::css::Css;

#[component]
pub fn Icon(icon: &'static str, size: &'static str) -> Element {
    rsx! {
        span { class: [Css::ICON, Css::BG, size, icon].concat() }
    }
}
