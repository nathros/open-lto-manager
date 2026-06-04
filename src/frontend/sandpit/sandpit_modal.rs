use dioxus::prelude::*;

use crate::frontend::{
    elements::{button::Button, input::InputType},
    modules::modal::{
        MODAL_SANDPIT_ERROR, MODAL_SANDPIT_INFO, MODAL_SANDPIT_SUCCESS, MODAL_SANDPIT_WARNING,
        Modal,
    },
};
use crate::shared::level::Level;

#[component]
pub fn SandpitModal() -> Element {
    let mut message_input = use_signal(|| "Test message".to_string());
    let mut message_error = use_signal(|| String::default());
    let mut message_warning = use_signal(|| String::default());
    let mut message_info = use_signal(|| String::default());
    let mut message_success = use_signal(|| String::default());

    rsx! {
        Modal {
            id: MODAL_SANDPIT_ERROR,
            level: Level::Error,
            message: message_error,
        }
        Modal {
            id: MODAL_SANDPIT_WARNING,
            level: Level::Warning,
            message: message_warning,
        }
        Modal {
            id: MODAL_SANDPIT_INFO,
            level: Level::Info,
            message: message_info,
        }
        Modal {
            id: MODAL_SANDPIT_SUCCESS,
            level: Level::Success,
            message: message_success,
        }

        input {
            r#type: InputType::Text.to_string(),
            oninput: move |evt: Event<FormData>| { message_input.set(evt.value()) },
            value: message_input(),
        }
        br {}
        br {}

        Button {
            onclick: move |_| async move {
                message_error.set(message_input());
            },
            text: "Show Error",
        }
        span { " " }
        Button {
            onclick: move |_| async move {
                message_warning.set(message_input());
            },
            text: "Show Warning",
        }
        span { " " }
        Button {
            onclick: move |_| async move {
                message_info.set(message_input());
            },
            text: "Show Info",
        }
        span { " " }
        Button {
            onclick: move |_| async move {
                message_success.set(message_input());
            },
            text: "Show Success",
        }
    }
}
