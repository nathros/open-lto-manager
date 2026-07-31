use dioxus::prelude::*;

use crate::{
    either,
    frontend::{
        collections::{
            message::{Message, MessageDetails},
            radio_pill::RadioPill,
        },
        elements::input::{Input, InputType},
        level::Level,
    },
    shared::models::select_option::SelectOption,
};

#[component]
pub fn SandpitMessage() -> Element {
    let mut message_input = use_signal(|| "Test message".to_string());
    let mut small = use_signal(|| false);

    rsx! {
        Input {
            type_: InputType::Text,
            label: "Message text".to_string(),
            oninput: move |evt: Event<FormData>| { message_input.set(evt.value()) },
            value: message_input(),
        }
        br {}
        RadioPill {
            name: "msg_size",
            label: "Message size".to_string(),
            options: ["Small", "Large"]
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    SelectOption {
                        id: i as i64,
                        label: s.to_string(),
                    }
                })
                .collect(),
            callback: use_callback(move |id: i64| {
                small.set(id == 0);
            }),
            selected: either!(small(), 0, 1),
        }
        br {}
        Message {
            small: small(),
            details: MessageDetails {
                level: Level::Error,
                text: message_input(),
            },
        }
        br {}
        Message {
            small: small(),
            details: MessageDetails {
                level: Level::Warning,
                text: message_input(),
            },
        }
        br {}
        Message {
            small: small(),
            details: MessageDetails {
                level: Level::Info,
                text: message_input(),
            },
        }
        br {}
        Message {
            small: small(),
            details: MessageDetails {
                level: Level::Success,
                text: message_input(),
            },
        }
    }
}
