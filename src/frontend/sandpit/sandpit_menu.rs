use dioxus::prelude::*;

use crate::{
    frontend::{
        components::menu::{
            component::Menu,
            menu_config::{MenuConfig, MenuGroup, MenuItemConfig},
        },
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
                    },
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "inner 2.2".to_string(),
                        link: Route::Home {}.to_string(),
                    },
                ],
            },
        ],
    });

    rsx! {
        div { style: "width: 15rem",
            Menu { config, current_route: "".to_string() }
        }
    }
}
