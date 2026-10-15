use dioxus::prelude::*;

use crate::frontend::{components::dropdown::dropdown_config::DropDownConfig, css::Css};

#[component]
pub fn DropDown(props: DropDownConfig) -> Element {
    let menu_id = [props.anchor_id, "_m"].concat();
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
