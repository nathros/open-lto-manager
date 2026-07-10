use dioxus::prelude::*;

use crate::{
    either,
    frontend::{css::Css, icons::Icons},
};

#[component]
pub fn Accordion(label: String, #[props] children: Element) -> Element {
    let mut open = use_signal(|| false);
    // Could use <details> but it not possible to animate without JavaScript, same for Menu {}
    // https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/details
    // "Unfortunately, at this time, there's no built-in way to animate the transition between open and closed"
    rsx! {
        div { class: [Css::ACCORDION_CONTAINER, either!(open(), Css::ACTIVE, "")].concat(),
            div {
                class: Css::HOVER,
                onclick: move |evt| {
                    evt.stop_propagation();
                    open.set(!open());
                },
                span { class: [Css::ICON, Icons::CHEVRON_RIGHT, Css::SM].concat() }
                span { "{label}" }
            }
            div { {children} }
        }
    }
}

#[component]
pub fn AccordionExtended(#[props] header: Element, #[props] children: Element) -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        div { class: [Css::ACCORDION_CONTAINER, either!(open(), Css::ACTIVE, "")].concat(),
            div {
                class: Css::HOVER,
                onclick: move |evt| {
                    evt.stop_propagation();
                    open.set(!open());
                },
                span { class: [Css::ICON, Icons::CHEVRON_RIGHT, Css::SM].concat() }
                {header}
            }
            div { {children} }
        }
    }
}
