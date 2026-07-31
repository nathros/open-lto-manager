use dioxus::prelude::*;

use crate::frontend::css::Css;

#[component]
pub fn HeaderExtraIcons(children: Element) -> Element {
    rsx! {
        div { class: Css::HEADER_ANCHOR_POSITION, {children} }
    }
}
