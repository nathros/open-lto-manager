use dioxus::prelude::*;

use crate::frontend::{
    components::card::Card,
    css::Css,
    sandpit::{
        sandpit_button::SandpitButton, sandpit_menu::SandpitMenu,
        sandpit_menu_item::SandpitMenuItem, sandpit_message::SandpitMessage,
        sandpit_modal::SandpitModal,
    },
};

#[component]
pub fn SandpitShowcase() -> Element {
    let list: Vec<(&str, Element)> = vec![
        ("Button", SandpitButton()),
        ("Modal", SandpitModal()),
        ("Message", SandpitMessage()),
        ("Menu Item", SandpitMenuItem()),
        ("Menu", SandpitMenu()),
    ];

    rsx! {
        div { class: Css::FLEX_COL, style: "align-items:unset",
            for (name , component) in list {
                Card { top_padding: false,
                    h3 { "{name}" }
                    div { {component} }
                }
            }
        }
    }
}
