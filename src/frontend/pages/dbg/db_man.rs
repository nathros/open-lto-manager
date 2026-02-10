use dioxus::{fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_manufacturer::list_manu,
    shared::models::database::model_manufacturer::RecordManufacturer,
};

#[component]
pub fn DBMan() -> Element {
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
                th { "name" }
            }
            {children}
        }
    }
}

#[component]
fn Inner() -> Element {
    let list_manu: Loader<Vec<RecordManufacturer>> = use_loader(list_manu)?;
    rsx! {
        for rec in list_manu.cloned() {
            tr {
                td { "{rec.id}" }
                td { "{rec.name}" }
            }
        }
    }
}
