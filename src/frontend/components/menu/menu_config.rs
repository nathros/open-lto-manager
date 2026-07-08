use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct MenuItemConfig {
    pub icon: String,
    pub label: String,
    pub link: String,
    //pub selected: bool,
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

impl MenuGroup {
    pub fn new(
        current_route: &String,
        icon: String,
        label: String,
        items: Vec<MenuItemConfig>,
    ) -> MenuGroup {
        MenuGroup {
            icon,
            label,
            open: items.iter().find(|i| i.link == *current_route).is_some(),
            items,
        }
    }
}

impl MenuConfig {
    pub fn toggle_group(&mut self, index: usize) {
        let original = {
            if let Some(group) = self.groups.get(index) {
                group.open
            } else {
                false
            }
        };
        self.groups.iter_mut().for_each(|g| g.open = false);
        if let Some(group) = self.groups.get_mut(index) {
            group.open = !original;
        }
    }
}
