use dioxus::prelude::*;

use crate::frontend::css::Css;

#[derive(Props, PartialEq, Clone)]
pub struct HeaderIconProps {
    #[props]
    button_id: &'static str,

    #[props]
    menu_id: &'static str,

    #[props]
    icon: &'static str,

    #[props]
    children: Element,
}

#[component]
pub fn HeaderIcon(props: HeaderIconProps) -> Element {
    rsx! {
        button {
            class: Css::HEADER_DROPDOWN,
            id: props.button_id,
            "popovertarget": props.menu_id,
            span { class: format!("{}{}", Css::ICON, props.icon) }
        }
        div {
            id: props.menu_id,
            "anchor": props.button_id,
            class: Css::HEADER_DROPDOWN_CONTENT,
            popover: "auto",
            {props.children}
        }
    }
}
