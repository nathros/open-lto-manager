use dioxus::prelude::*;

use crate::{
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

#[derive(Props, PartialEq, Clone)]
pub struct MenuItemConfig {
    pub icon: String,
    pub label: String,
    pub link: String,
    pub selected: bool,
}

#[derive(Props, PartialEq, Clone)]
pub struct MenuGroup {
    pub icon: String,
    pub label: String,
    pub open: bool,
    pub items: Vec<MenuItemConfig>,
}

#[derive(Props, PartialEq, Clone)]
pub struct MenuConfig {
    pub enable_search: bool,
    pub groups: Vec<MenuGroup>,
}

impl MenuConfig {
    pub fn toggle_group(&mut self, index: usize) {
        if let Some(group) = self.groups.get_mut(index) {
            group.open = !group.open;
        }
    }
    pub fn set_selected(&mut self, index_group: usize, index_item: usize) {
        // Reset current selected
        self.groups
            .iter_mut()
            .for_each(|g| g.items.iter_mut().for_each(|i| i.selected = false));

        if let Some(group) = self.groups.get_mut(index_group)
            && let Some(item) = group.items.get_mut(index_item)
        {
            item.selected = true;
        }
    }
}

#[component]
pub fn Menu(config: Signal<MenuConfig>) -> Element {
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
                        div { class: static_concat!(Css::MENU_GROUP),
                            if group.open {
                                for (i_index , item) in group.items.iter().enumerate() {
                                    if let link = item.link.clone() {
                                        MenuItem {
                                            onclick: move |evt: MouseEvent| {
                                                evt.stop_propagation();
                                                config.write().set_selected(g_index, i_index);
                                                fn_link_follow(link.clone());
                                            },
                                            text: item.label.to_owned(),
                                            selected: item.selected,
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if let mut count = 0 {
                        for (g_index , group) in c.groups.iter().enumerate() {
                            for (i_index , item) in group.items.iter().enumerate() {
                                if let link = item.link.clone() && item.label.to_lowercase().contains(&filter()) {
                                    MenuItem {
                                        onclick: move |evt: MouseEvent| {
                                            evt.stop_propagation();
                                            config.write().set_selected(g_index, i_index);
                                            fn_link_follow(link.clone());
                                        },
                                        text: item.label.to_owned(),
                                        selected: item.selected,
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
