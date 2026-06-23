use dioxus::prelude::*;

use crate::{
    frontend::{
        components::menu::{Menu, MenuConfig, MenuGroup, MenuItemConfig},
        icons::Icons,
    },
    route::Route,
};

#[component]
pub fn SandpitMenu() -> Element {
    let config = use_signal(|| MenuConfig {
        enable_search: true,
        groups: vec![
            MenuGroup {
                icon: Icons::ERROR.into(),
                label: "label 1".to_string(),
                open: false,
                items: vec![MenuItemConfig {
                    icon: "".to_string(),
                    label: "inner 1.1".to_string(),
                    link: Route::Home {}.to_string(),
                    selected: false,
                }],
            },
            MenuGroup {
                icon: Icons::SANDPIT.into(),
                label: "label 2".to_string(),
                open: false,
                items: vec![
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "inner 2.1".to_string(),
                        link: Route::Home {}.to_string(),
                        selected: false,
                    },
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "inner 2.2".to_string(),
                        link: Route::Home {}.to_string(),
                        selected: false,
                    },
                ],
            },
        ],
    });

    rsx! {
        div { style: "width: 15rem",
            Menu { config }
        }
    }
}
