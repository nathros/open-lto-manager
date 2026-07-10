use dioxus::prelude::*;

use crate::{either, frontend::css::Css};

#[component]
pub fn H1(
    #[props(optional, default = false)] margin: bool,
    #[props(extends = button, extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(optional, default = None)] children: Option<Element>,
) -> Element {
    rsx! {
        h1 { class: either!(margin, Css::REVERT, ""), ..attributes, {children} }
    }
}

#[component]
pub fn H2(
    #[props(optional, default = false)] margin: bool,
    #[props(extends = button, extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(optional, default = None)] children: Option<Element>,
) -> Element {
    rsx! {
        h2 { class: either!(margin, Css::REVERT, ""), ..attributes, {children} }
    }
}

#[component]
pub fn H3(
    #[props(optional, default = false)] margin: bool,
    #[props(extends = button, extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(optional, default = None)] children: Option<Element>,
) -> Element {
    rsx! {
        h3 { class: either!(margin, Css::REVERT, ""), ..attributes, {children} }
    }
}

#[component]
pub fn H4(
    #[props(optional, default = false)] margin: bool,
    #[props(extends = button, extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(optional, default = None)] children: Option<Element>,
) -> Element {
    rsx! {
        h4 { class: either!(margin, Css::REVERT, ""), ..attributes, {children} }
    }
}
