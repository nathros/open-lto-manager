use dioxus::prelude::*;

use crate::frontend::{components::dropdown::dropdown_config::DropDownConfig, css::Css};

#[component]
pub fn DropDown(props: DropDownConfig) -> Element {
    let menu_id = format!("{}_m", props.anchor_id);
    rsx! {
        button {
            class: Css::BTN_RESET,
            id: props.anchor_id,
            "popovertarget": menu_id.clone(),
            {props.anchor}
        }
        div {
            id: menu_id,
            "anchor": props.anchor_id,
            class: Css::HEADER_DROPDOWN_CONTENT,
            popover: "auto",
            {props.children}
        }
    }
}
