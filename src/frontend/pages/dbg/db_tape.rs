use dioxus::{fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_tape::{api_del_tape, list_tape},
    frontend::{collections::message::Message, elements::button::Button},
    shared::{
        level::Level,
        models::database::model_tape::{
            EncryptionType, HardwareEncryptionType, RecordTape, SoftwareEncryptionType, TapeFormat,
        },
    },
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
    let mut message: Signal<String> = use_signal(|| String::default());

    rsx! {
        if let Some(e) = tapes_list.error() {
            p { "Failed with error: {e}" }
        } else {
            Message { level: Level::Error, text: message() }
            Table {
                for rec in tapes_list.cloned() {
                    tr {
                        td { "{rec.id}" }
                        td { "{rec.manufacturer_id}" }
                        td { "{rec.tape_type_id}" }
                        td { "{rec.barcode}" }
                        td { "{rec.serial}" }
                        td { "{<TapeFormat as Into<&str>>::into(rec.format)}" }
                        td { "{rec.worm}" }
                        td { "{<EncryptionType as Into<&str>>::into(rec.encryption_type)}" }
                        td { "{<SoftwareEncryptionType as Into<&str>>::into(rec.encryption_sw)}" }
                        td { "{<HardwareEncryptionType as Into<&str>>::into(rec.encryption_hw)}" }
                        td { "{rec.compressed}" }
                        td { "{rec.used_space}" }
                        td { "{rec.created}" }
                        td { "{rec.last_used}" }
                        td {
                            Button {
                                onclick: move |_| async move {
                                    match api_del_tape(rec.id).await {
                                        Ok(_) => {
                                            message.write().clear();
                                            tapes_list.restart();
                                        }
                                        Err(e) => message.set(format!("{}", e)),
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
