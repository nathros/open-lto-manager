use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct DropDownConfig {
    #[props]
    pub anchor_id: &'static str,

    #[props]
    pub anchor: Element,

    #[props]
    pub children: Element,
}
