use dioxus::{fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_job_metadata::list_metadata,
    shared::models::database::model_job_metadata::RecordJobMetadata,
};
#[component]
pub fn DBJobMetaData() -> Element {
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
                th { "job_id" }
                th { "key" }
                th { "index" }
                th { "value" }
            }
            {children}
        }
    }
}

#[component]
fn Inner() -> Element {
    let files_list: Loader<Vec<RecordJobMetadata>> = use_loader(list_metadata)?;

    rsx! {
        for rec in files_list.cloned() {
            tr {
                td { "{rec.id}" }
                td { "{rec.job_id}" }
                td { "{rec.key:?}" }
                td { "{rec.index}" }
                td { "{rec.value}" }
            }
        }
    }
}
