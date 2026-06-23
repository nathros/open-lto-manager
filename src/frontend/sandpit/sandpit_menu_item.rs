use dioxus::prelude::*;

use crate::{
    frontend::{css::Css, elements::menu_item::MenuItem, icons::Icons, utils::function::fn_link},
    route::Route,
    static_concat,
};

#[component]
pub fn SandpitMenuItem() -> Element {
    rsx! {
        div { class: Css::FLEX_ROW,
            section {
                class: static_concat!(Css::FLEX_COL, Css::NO_GAP),
                style: "width: 20rem",
                MenuItem { text: "Menu item 1" }
                MenuItem { text: "Menu item 2 (selected)", selected: true }
                MenuItem {
                    text: "Menu item 3 (has child)",
                    children: rsx! {
                        b { "child" }
                    },
                }
                MenuItem {
                    text: "Menu item 4 (link)",
                    onclick: fn_link(Route::Home {}.to_string()),
                }
            }
            section {
                class: static_concat!(Css::FLEX_COL, Css::NO_GAP),
                style: "width: 20rem",
                MenuItem { icon: Icons::USER, text: "Menu item 1" }
                MenuItem {
                    icon: Icons::LOGOUT,
                    text: "Menu item 2 (selected)",
                    selected: true,
                }
                MenuItem {
                    icon: Icons::PALETTE,
                    text: "Menu item 3 (has child)",
                    children: rsx! {
                        b { "child" }
                    },
                }
                MenuItem {
                    icon: Icons::WARNING,
                    text: "Menu item 4 (link)",
                    onclick: fn_link(Route::Home {}.to_string()),
                }
            }
        }
    }
}
