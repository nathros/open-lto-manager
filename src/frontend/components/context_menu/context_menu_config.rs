use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ContextMenuConfig {
    #[props]
    pub anchor_id: &'static str,

    #[props]
    pub children: Element,
}
