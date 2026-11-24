use dioxus::prelude::*;

use crate::{
    either,
    frontend::{components::dropdown::dropdown_config::DropDownConfig, css::Css},
};

#[component]
pub fn DropDown(props: DropDownConfig) -> Element {
    let menu_id = [props.anchor_id, "_m"].concat();
    rsx! {
        button {
            id: props.anchor_id,
            class: Css::BTN_RESET,
            "popovertarget": menu_id.clone(),
            {props.anchor}
        }
        div {
            id: menu_id,
            class: [Css::HEADER_DROPDOWN_CONTENT, either!(props.top, Css::TOP, "")].concat(),
            "anchor": props.anchor_id,
            popover: "auto",
            {props.children}
        }
    }
}
