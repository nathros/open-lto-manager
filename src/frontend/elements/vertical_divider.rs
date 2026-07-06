use dioxus::prelude::*;

use crate::frontend::css::Css;

#[component]
pub fn VerticalDivider() -> Element {
    rsx! {
        div { class: Css::V_DIVIDER }
    }
}
