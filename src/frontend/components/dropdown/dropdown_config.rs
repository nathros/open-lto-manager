use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct DropDownConfig {
    #[props]
    pub anchor_id: &'static str,

    #[props]
    pub anchor: Element,

    // Menu anchor at top
    #[props(default = false)]
    pub top: bool,

    #[props]
    pub children: Element,
}
