use dioxus::prelude::*;

use crate::frontend::elements::button::Button;
use crate::shared::level::{Level, level_style};

#[component]
pub fn Modal(id: String, level: Level, message: Signal<String>) -> Element {
    let id_cpy = id.clone();
    use_effect(move || {
        if !message().is_empty() {
            to_owned![id_cpy];
            spawn(async move {
                let script = format!("document.getElementById('{}').showModal();", id_cpy);
                let _eval = document::eval(script.as_str()).await;
            });
        }
    });

    rsx! {
        dialog { id: id.clone(), style: level_style(level),
            p { "{message}" }
            br {}
            br {}
            Button {
                r#type: "button",
                "commandfor": id.clone(),
                "command": "close",
                onclick: move |_| {
                    message.set("".to_string());
                },
                text: "OK",
            }
        }
    }
}
