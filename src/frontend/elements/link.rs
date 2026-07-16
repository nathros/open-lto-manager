use dioxus::prelude::*;

use crate::frontend::css::Css;

#[component]
pub fn InlineLink(
    #[props(extends = a, extends = GlobalAttributes)] attributes: Vec<Attribute>,
    label: &'static str,
) -> Element {
    rsx! {
        a { class: Css::LINK, ..attributes, {label} }
    }
}
