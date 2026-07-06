use dioxus::prelude::*;

use crate::{either, frontend::css::Css, static_concat};

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    #[props(default = "".into())]
    style: String,

    #[props(default = "".into())]
    text: String,

    #[props(optional, default = false)]
    primary: bool,

    #[props(optional, default = false)]
    disabled: bool,

    #[props(optional)]
    onclick: EventHandler<MouseEvent>,

    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[derive(Props, PartialEq, Clone)]
pub struct LinkButtonProps {
    #[props(default = "".into())]
    style: String,

    #[props(default = "".into())]
    text: String,

    #[props(optional, default = false)]
    primary: bool,

    #[props]
    to: NavigationTarget,
}

/*pub enum ButtonType {
    button, // HTML default
    submit,
    reset,
}

impl ButtonType {
    pub fn to_string(&self) -> &str {
        match self {
            ButtonType::button => "button",
            ButtonType::submit => "submit",
            ButtonType::reset => "reset",
        }
    }
}*/

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: either!(props.primary, static_concat!(Css::BTN, Css::BTN_PRI), Css::BTN),
            style: props.style,
            onclick: props.onclick,
            disabled: props.disabled,
            ..props.attributes,
            "{props.text}"
        }
    }
}

#[component]
pub fn LinkButton(props: LinkButtonProps) -> Element {
    rsx! {
        Link {
            class: either!(props.primary, static_concat!(Css::BTN, Css::BTN_PRI), Css::BTN),
            style: props.style,
            to: props.to,
            "{props.text}"
        }
    }
}
