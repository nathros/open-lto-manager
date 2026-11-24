use dioxus::prelude::*;

use crate::frontend::{css::Css, elements::button::Button, icons::Icons};

#[derive(Props, PartialEq, Clone)]
pub struct DialogProps {
    #[props]
    id: &'static str,

    #[props(default = false)]
    button_bar: bool,

    #[props(default = false)]
    button_bar_save: bool,

    #[props(default = false)]
    button_bar_save_disabled: bool,

    #[props(default = "Save")]
    button_bar_save_text: &'static str,

    #[props(optional)]
    button_bar_save_click: EventHandler<MouseEvent>,

    #[props(default = false)]
    button_bar_close: bool,

    #[props(default = "Cancel")]
    button_bar_close_text: &'static str,

    #[props]
    children: Element,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    rsx! {
        dialog { id: props.id,
            {props.children}
            if props.button_bar {
                div { class: Css::BTN_BAR,
                    if props.button_bar_save {
                        Button {
                            primary: true,
                            icon: Icons::SAVE,
                            disabled: props.button_bar_save_disabled,
                            onclick: props.button_bar_save_click,
                            "commandfor": props.id,
                            "command": "close",
                            text: props.button_bar_save_text,
                        }
                    }
                    if props.button_bar_close {
                        Button {
                            icon: Icons::X_MARK,
                            "commandfor": props.id,
                            "command": "close",
                            text: props.button_bar_close_text,
                        }
                    }
                }
            }
        }
    }
}
