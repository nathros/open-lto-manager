use dioxus::{fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_tape::list_tape,
    shared::models::database::model_tape::{RecordTape, TapeFormat},
};

#[component]
pub fn DBTape() -> Element {
    let tapes_list: Loader<Vec<RecordTape>> = use_loader(list_tape)?;

    rsx! {
        if let Some(e) = tapes_list.error() {
            p { "Failed with error: {e}" }
        } else {
            table {
                tr {
                    th { "id" }
                    th { "manufacturer_id" }
                    th { "tape_type_id" }
                    th { "barcode" }
                    th { "serial" }
                    th { "format" }
                    th { "worm" }
                    th { "encrypted" }
                    th { "compressed" }
                    th { "used_space" }
                    th { "created" }
                    th { "last_used" }
                }
                for rec in tapes_list.cloned() {
                    tr {
                        td { "{rec.id}" }
                        td { "{rec.manufacturer_id}" }
                        td { "{rec.tape_type_id}" }
                        td { "{rec.barcode}" }
                        td { "{rec.serial}" }
                        td { "{<TapeFormat as Into<&str>>::into(rec.format)}" }
                        td { "{rec.worm}" }
                        td { "{rec.encrypted}" }
                        td { "{rec.compressed}" }
                        td { "{rec.used_space}" }
                        td { "{rec.created}" }
                        td { "{rec.last_used}" }
                    }
                }
            }
        }
    }
}
