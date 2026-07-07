use dioxus::prelude::*;

use crate::frontend::{
    elements::{
        button::Button,
        input::{Input, InputType},
    },
    id::Id,
    level::Level,
    modules::modal::Modal,
};

#[component]
pub fn SandpitModal() -> Element {
    let mut message_input = use_signal(|| "Test message".to_string());
    let mut message_error = use_signal(|| String::default());
    let mut message_warning = use_signal(|| String::default());
    let mut message_info = use_signal(|| String::default());
    let mut message_success = use_signal(|| String::default());

    rsx! {
        Modal {
            id: Id::ModalSandpitError.as_str(),
            level: Level::Error,
            message: message_error,
        }
        Modal {
            id: Id::ModalSandpitWarning.as_str(),
            level: Level::Warning,
            message: message_warning,
        }
        Modal {
            id: Id::ModalSandpitInfo.as_str(),
            level: Level::Info,
            message: message_info,
        }
        Modal {
            id: Id::ModalSandpitSuccess.as_str(),
            level: Level::Success,
            message: message_success,
        }

        Input {
            type_: InputType::Text,
            oninput: move |evt: Event<FormData>| { message_input.set(evt.value()) },
            value: message_input(),
        }
        br {}

        Button {
            onclick: move |_evt: MouseEvent| async move {
                message_error.set(message_input());
            },
            text: "Show Error",
        }
        span { " " }
        Button {
            onclick: move |_evt: MouseEvent| async move {
                message_warning.set(message_input());
            },
            text: "Show Warning",
        }
        span { " " }
        Button {
            onclick: move |_evt: MouseEvent| async move {
                message_info.set(message_input());
            },
            text: "Show Info",
        }
        span { " " }
        Button {
            onclick: move |_evt: MouseEvent| async move {
                message_success.set(message_input());
            },
            text: "Show Success",
        }
    }
}
