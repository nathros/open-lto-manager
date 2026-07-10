use dioxus::prelude::*;

use crate::{
    either,
    frontend::{
        css::Css,
        elements::heading::{H2, H4},
    },
};

#[component]
pub fn Card(
    children: Element,
    #[props(optional, default = true)] top_padding: bool,
    #[props(extends = div, extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        if top_padding {
            div { class: Css::CARD, ..attributes, {children} }
        } else {
            div { style: "padding-top:0", class: Css::CARD, ..attributes, {children} }
        }
    }
}

#[component]
pub fn CardOverview(
    children: Element,
    title: String,
    #[props(extends = div, extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div { class: Css::CARD_OVERVIEW, ..attributes,
            H2 { margin: true, "{title}" }
            div { {children} }
        }
    }
}

#[component]
pub fn CardOverviewStatus(class: &'static str, title: &'static str, count: i32) -> Element {
    let set_class = either!(count > 0, class, "");
    let bg = either!(count > 0, Css::BG, "");
    rsx! {
        div { class: [Css::CARD, Css::CARD_OVERVIEW_STATUS, Css::COL, set_class].concat(),
            H4 { "{title}" }
            div {
                H2 { "{count}" }
                span { class: [Css::ICON, bg, class].concat() }
            }
        }
    }
}
