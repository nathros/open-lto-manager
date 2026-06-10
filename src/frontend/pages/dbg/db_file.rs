use dioxus::{fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_file::list_files, shared::models::database::file::model_file::RecordFile,
};

#[component]
pub fn DBFile() -> Element {
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
                th { "tape_id" }
                th { "file_name_virt" }
                th { "file_path_virt" }
                th { "file_name_phy" }
                th { "file_path_phy" }
                th { "file_size" }
                th { "created" }
                th { "modified" }
                th { "hash" }
                th { "icon" }
            }
            {children}
        }
    }
}

#[component]
fn Inner() -> Element {
    let files_list: Loader<Vec<RecordFile>> = use_loader(list_files)?;

    rsx! {
        for rec in files_list.cloned() {
            tr {
                td { "{rec.id}" }
                if let Some(tape_id) = rec.tape_id {
                    td { "{tape_id}" }
                } else {
                    td { "null" }
                }
                td { "{rec.file_name_virt}" }
                td { "{rec.file_path_virt}" }
                td { "{rec.file_name_phy}" }
                td { "{rec.file_path_phy}" }
                td { "{rec.file_size}" }
                td { "{rec.created}" }
                td { "{rec.modified}" }
                td { "{rec.hash}" }
                td { "{rec.icon}" }
            }
        }
    }
}
