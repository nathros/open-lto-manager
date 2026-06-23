use dioxus::prelude::*;

use crate::frontend::{css::Css, elements::button::Button, level::Level};

// Only 1 reference per ID to avoid duplicate ids
pub const MODAL_JOB_ID: &str = "modal-b_job";
pub const MODAL_SANDPIT_ERROR: &str = "modal-sandpit_e";
pub const MODAL_SANDPIT_WARNING: &str = "modal-sandpit_w";
pub const MODAL_SANDPIT_INFO: &str = "modal-sandpit_i";
pub const MODAL_SANDPIT_SUCCESS: &str = "modal-sandpit_s";

#[component]
pub fn Modal(id: &'static str, level: Level, message: Signal<String>) -> Element {
    use_effect(move || {
        if !message().is_empty() {
            spawn(async move {
                let script = format!("document.getElementById('{}').showModal();", id);
                let _eval = document::eval(script.as_str()).await;
            });
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
