use dioxus::prelude::*;

use crate::{
    either,
    frontend::{
        assets::LOGO_ASSET,
        css::Css,
        elements::{
            input::{Input, InputType},
            menu_item::MenuItem,
        },
        js::js_system_canvas,
        utils::function::fn_link_follow,
    },
    static_concat,
};

use super::menu_config::MenuConfig;

#[component]
pub fn Menu(config: Signal<MenuConfig>, current_route: String) -> Element {
    let mut filter: Signal<String> = use_signal(|| "".to_string());

    rsx! {
        div { class: static_concat!(Css::FLEX_COL, Css::NO_GAP, Css::MENU_CONTAINER),
            if let c = config() {
                if c.enable_search {
                    Input {
                        type_: InputType::Search,
                        oninput: move |evt: Event<FormData>| {
                            filter.set(evt.value().to_lowercase());
                        },
                        value: filter(),
                        placeholder: "search",
                    }
                }
                if filter().is_empty() {
                    for (g_index , group) in c.groups.iter().enumerate() {
                        MenuItem {
                            onclick: move |_evt: MouseEvent| {
                                config.write().toggle_group(g_index);
                            },
                            icon: group.icon.clone(),
                            text: group.label.clone(),
                            open: group.open,
                            vertical: true,
                            children: rsx! {},
                        }
                        div {
                            class: Css::MENU_GROUP,
                            style: format!("max-height:{}px", either!(group.open, 35 * group.items.len(), 0)),
                            for item in group.items.iter() {
                                if let link = item.link.clone() {
                                    MenuItem {
                                        onclick: move |evt: MouseEvent| {
                                            evt.stop_propagation();
                                            fn_link_follow(link.clone());
                                        },
                                        text: item.label.to_owned(),
                                        selected: item.link == current_route,
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if let mut count = 0 {
                        for group in c.groups.iter() {
                            for item in group.items.iter() {
                                if let link = item.link.clone() && item.label.to_lowercase().contains(&filter()) {
                                    MenuItem {
                                        onclick: move |evt: MouseEvent| {
                                            evt.stop_propagation();
                                            fn_link_follow(link.clone());
                                        },
                                        text: item.label.to_owned(),
                                        selected: item.link == current_route,
                                    }
                                    {
                                        count += 1;
                                    }
                                }
                            }
                        }
                        if count == 0 {
                            p { "No results" }
                            div { class: static_concat!(Css::FLEX_CENTRE, Css::MD),
                                img {
                                    id: Css::ID_SNA,
                                    src: format!("{}#sna", LOGO_ASSET),
                                    onclick: move |_: MouseEvent| {
                                        js_system_canvas();
                                    },
                                }
                                i { "{MSG}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

const MSG: &str = {
    const fn _f(i: &[u8]) -> &str {
        let Ok(s) = std::str::from_utf8(i) else {
            unreachable!();
        };
        s
    }
    _f
}(&[
    0x53u8, 0x6eu8, 0x61u8, 0x6bu8, 0x65u8, 0x2eu8, 0x2eu8, 0x2eu8,
]);
