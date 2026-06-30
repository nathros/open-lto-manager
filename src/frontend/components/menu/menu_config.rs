use dioxus::prelude::*;

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
