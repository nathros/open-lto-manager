use dioxus::{fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_tape::{api_del_tape, list_tape},
    frontend::{
        collections::message::{Message, MessageDetails},
        elements::button::Button,
    },
    shared::models::database::tape::model_tape::RecordTape,
};

#[component]
pub fn DBTape() -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |_suspense_context: SuspenseContext| {
                rsx! {
                    Table {}
                }
            },
            Inner {}
        }
    }
}

#[component]
fn Table(children: Element) -> Element {
    rsx! {
        table {
            tr {
                th { "id" }
                th { "manufacturer_id" }
                th { "tape_type_id" }
                th { "barcode" }
                th { "serial" }
                th { "format" }
                th { "worm" }
                th { "encryption_type" }
                th { "encryption_sw" }
                th { "encryption_hw" }
                th { "compressed" }
                th { "used_space" }
                th { "created" }
                th { "last_used" }
                th { "Action" }
            }
            {children}
        }
    }
}

#[component]
fn Inner() -> Element {
    let mut tapes_list: Loader<Vec<RecordTape>> = use_loader(list_tape)?;
    let mut message: Signal<MessageDetails> = use_signal(|| MessageDetails::default());

    rsx! {
        if let Some(e) = tapes_list.error() {
            p { "Failed with error: {e}" }
        } else {
            Message { details: message() }
            Table {
                for rec in tapes_list.cloned() {
                    tr {
                        td { "{rec.id}" }
                        td { "{rec.manufacturer_id}" }
                        td { "{rec.tape_type_id}" }
                        td { "{rec.barcode}" }
                        td { "{rec.serial:?}" }
                        td { "{rec.format:?}" }
                        td { "{rec.worm}" }
                        td { "{rec.encryption_type:?}" }
                        td { "{rec.encryption_sw:?}" }
                        td { "{rec.encryption_hw:?}" }
                        td { "{rec.compressed}" }
                        td { "{rec.used_space}" }
                        td { "{rec.created}" }
                        td { "{rec.last_used}" }
                        td {
                            Button {
                                onclick: move |_evt: MouseEvent| async move {
                                    match api_del_tape(rec.id).await {
                                        Ok(_) => {
                                            message.write().text.clear();
                                            tapes_list.restart();
                                        }
                                        Err(e) => message.write().text = format!("{}", e),
                                    }
                                },
                                text: "Delete",
                            }
                        }
                    }
                }
            }
        }
    }
}
