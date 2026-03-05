use dioxus::{fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_devices::list_tape_devices,
    frontend::collections::message::Message,
    shared::{level::Level, models::tape_drive::TapeDrive},
};

#[component]
pub fn ShowDevices() -> Element {
    let list_result: Loader<Result<Vec<TapeDrive>, String>> = use_loader(list_tape_devices)?;

    rsx! {
        match list_result() {
            Ok(list) => rsx! {
                if list.is_empty() {
                    span { "None found" }
                }
                for l in list {
                    span { "{l.dev} : {l.manufacturer}" }
                    br {}
                }
            },
            Err(e) => rsx! {
                Message { level: Level::Error, text: e }
            },
        }
    }
}
