use dioxus::prelude::*;

use crate::frontend::{css::Css, elements::button::Button, js::js_show_modal, level::Level};

#[component]
pub fn Modal(id: &'static str, level: Level, message: Signal<String>) -> Element {
    use_effect(move || {
        if !message().is_empty() {
            js_show_modal(id);
        }
    });

    rsx! {
        dialog { id, class: level.to_class(),
            div { class: Css::FLEX_ROW,
                div { class: [Css::ICON, Css::BG, level.to_class()].concat() }
                p { "{message}" }
            }
            br {}
            Button {
                "commandfor": id,
                "command": "close",
                onclick: move |_evt: MouseEvent| {
                    message.set("".to_string());
                },
                text: "OK",
            }

        }
    }
}
