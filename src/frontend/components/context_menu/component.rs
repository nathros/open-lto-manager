use dioxus::prelude::*;

use crate::{
    frontend::{components::context_menu::context_menu_config::ContextMenuConfig, css::Css},
    static_concat,
};

#[component]
pub fn ContextMenu(props: ContextMenuConfig) -> Element {
    let menu_id = [props.anchor_id, "_m"].concat();
    rsx! {
        button {
            id: props.anchor_id,
            class: static_concat!(Css::CONTEXT_MENU_ANCHOR, Css::HIDDEN_BUTTON_ANCHOR),
            "popovertarget": menu_id.clone(),
        }
        div {
            id: menu_id,
            class: static_concat!(Css::CONTEXT_MENU, Css::HEADER_DROPDOWN_CONTENT),
            "anchor": props.anchor_id,
            popover: "auto",
            {props.children}
        }
    }
}
