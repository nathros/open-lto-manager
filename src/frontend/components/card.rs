use dioxus::prelude::*;

use crate::frontend::css::Css;

#[component]
pub fn Card(children: Element, #[props(optional, default = true)] top_padding: bool) -> Element {
    rsx! {
        if top_padding {
            div { class: Css::CARD, {children} }
        } else {
            div { style: "padding-top:0", class: Css::CARD, {children} }
        }
    }
}
