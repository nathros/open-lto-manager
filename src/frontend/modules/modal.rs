use dioxus::prelude::*;

use crate::frontend::elements::button::Button;
use crate::shared::level::Level;

// Only 1 reference to avoid duplicate ids
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
        dialog { id, style: level.to_style(),
            p { "{message}" }
            br {}
            br {}
            Button {
                r#type: "button",
                "commandfor": id,
                "command": "close",
                onclick: move |_| {
                    message.set("".to_string());
                },
                text: "OK",
            }
        }
    }
}
