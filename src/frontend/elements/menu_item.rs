use dioxus::prelude::*;

use crate::{
    either,
    frontend::{css::Css, icons::Icons},
    static_concat,
};

#[derive(Props, PartialEq, Clone)]
pub struct MenuItemProps {
    #[props]
    text: String,

    #[props(default = "".into())]
    icon: String,

    #[props(optional, default = false)]
    selected: bool,

    #[props(optional, default = false)]
    vertical: bool,

    #[props(optional, default = false)]
    open: bool,

    #[props(optional)]
    onclick: EventHandler<MouseEvent>,

    #[props(optional, default = None)]
    children: Option<Element>,
}

#[component]
pub fn MenuItem(props: MenuItemProps) -> Element {
    rsx! {
        div {
            class: [
                Css::ICON_LIST_ITEM,
                Css::FLEX_ROW,
                Css::FLEX_ALIGN_CENTRE,
                either!(props.selected, Css::SELECTED, ""),
            ]
                .concat(),
            onclick: props.onclick,
            if !props.icon.is_empty() {
                span { class: [Css::ICON, Css::SM, props.icon.as_str()].concat() }
            }
            span { "{props.text}" }
            if let Some(children) = props.children {
                span {
                    class: static_concat!(Css::ICON, Css::SM, Css::FLOAT_RIGHT, Icons::CHEVRON_RIGHT),
                    style: [
                        "transform:",
                        either!(props.vertical, "rotate(90deg)", ""),
                        either!(props.open, " scaleX(-1)", ""),
                    ]
                        .concat(),
                }
                {children}
            }
        }
    }
}
