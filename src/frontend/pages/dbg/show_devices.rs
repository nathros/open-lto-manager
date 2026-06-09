use dioxus::{fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_devices::{list_tape_devices, text_stream3},
    frontend::{
        collections::message::{Message, MessageDetails},
        level::Level,
    },
    shared::models::tape_drive::TapeDrive,
};

#[component]
pub fn ShowDevices() -> Element {
    let list_result: Loader<Result<Vec<TapeDrive>, String>> = use_loader(list_tape_devices)?;

    let mut stream_output: Signal<Vec<String>> = use_signal(|| vec![]);
    use_future(move || async move {
        match text_stream3(2).await {
            Ok(mut stream) => {
                while let Some(Ok(text)) = stream.next().await {
                    stream_output.write().push(text);
                }
            }
            Err(e) => error!("UI {}", e),
        }
    });

    rsx! {
        match list_result() {
            Ok(list) => rsx! {
                if list.is_empty() {
                    span { "None found in list" }
                }
                br {}
                for l in list {
                    span { "{l.dev} : {l.manufacturer}" }
                    br {}
                }
            },
            Err(e) => rsx! {
                Message {
                    details: MessageDetails {
                        level: Level::Error,
                        text: e,
                    },
                }
            },
        }
        for i in stream_output.read().iter() {
            span { "{i}" }
            hr {}
        }
    }
}
