use dioxus::{fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_tape_type::list_type_type,
    shared::models::database::tape_type::model_tape_type::RecordTapeType,
};

#[component]
pub fn DBType() -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |_suspense_context: SuspenseContext| {
                rsx! {
                    Table {}
                }
            },
            Table { Inner {} }
        }
    }
}

#[component]
fn Table(children: Element) -> Element {
    rsx! {
        table {
            tr {
                th { "id" }
                th { "generation" }
                th { "description" }
                th { "id_reg" }
                th { "id_worm" }
                th { "native_capacity" }
                th { "colour_reg" }
                th { "colour_hp" }
                th { "colour_worm_reg" }
                th { "colour_worm_hp" }
                th { "supports_worm" }
                th { "supports_encryption" }
                th { "supports_ltfs" }
            }
            {children}
        }
    }
}

#[component]
fn Inner() -> Element {
    let tapes_list: Loader<Vec<RecordTapeType>> = use_loader(list_type_type)?;

    rsx! {
        for rec in tapes_list.cloned() {
            tr {
                td { "{rec.id}" }
                td { "{rec.generation}" }
                td { "{rec.description}" }
                td { "{rec.id_reg}" }
                td { "{rec.id_worm}" }
                td { "{rec.native_capacity}" }
                td { "{rec.colour_reg}" }
                td { "{rec.colour_hp}" }
                td { "{rec.colour_worm_reg}" }
                td { "{rec.colour_worm_hp}" }
                td { "{rec.supports_worm}" }
                td { "{rec.supports_encryption}" }
                td { "{rec.supports_ltfs}" }
            }
        }

    }
}
