use dioxus::prelude::*;

use crate::frontend::{
    collections::message::{Message, MessageDetails},
    elements::input::{Input, InputType},
    level::Level,
};

#[component]
pub fn SandpitMessage() -> Element {
    let mut message_input = use_signal(|| "Test message".to_string());

    rsx! {
        Input {
            type_: InputType::Text,
            oninput: move |evt: Event<FormData>| { message_input.set(evt.value()) },
            value: message_input(),
        }
        br {}
        Message {
            details: MessageDetails {
                level: Level::Error,
                text: message_input(),
            },
        }
        br {}
        Message {
            details: MessageDetails {
                level: Level::Warning,
                text: message_input(),
            },
        }
        br {}
        Message {
            details: MessageDetails {
                level: Level::Info,
                text: message_input(),
            },
        }
        br {}
        Message {
            details: MessageDetails {
                level: Level::Success,
                text: message_input(),
            },
        }
    }
}
