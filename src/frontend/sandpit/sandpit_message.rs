use dioxus::prelude::*;

use crate::frontend::{collections::message::Message, elements::input::InputType, level::Level};

#[component]
pub fn SandpitMessage() -> Element {
    let mut message_input = use_signal(|| "Test message".to_string());

    rsx! {
        input {
            r#type: InputType::Text.to_string(),
            oninput: move |evt: Event<FormData>| { message_input.set(evt.value()) },
            value: message_input(),
        }
        br {}
        br {}
        Message { level: Level::Error, text: message_input() }
        br {}
        Message { level: Level::Warning, text: message_input() }
        br {}
        Message { level: Level::Info, text: message_input() }
        br {}
        Message { level: Level::Success, text: message_input() }
    }
}
