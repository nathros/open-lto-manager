use dioxus::{fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_job::{delete_job, list_jobs},
    frontend::{
        collections::message::{Message, MessageDetails},
        elements::button::Button,
    },
    shared::models::database::model_job::RecordJob,
};

#[component]
pub fn DBJob() -> Element {
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
                th { "user_id" }
                th { "name" }
                th { "job_type" }
                th { "job_status" }
                th { "start_time" }
                th { "end_time" }
                th { "comment" }
                th { "Action" }
            }
            {children}
        }
    }
}

#[component]
fn Inner() -> Element {
    let mut jobs_list: Loader<Vec<RecordJob>> = use_loader(list_jobs)?;
    let mut msg: Signal<MessageDetails> = use_signal(|| MessageDetails::default());

    rsx! {
        Message { details: msg() }
        for rec in jobs_list.cloned() {

            tr {
                td { "{rec.id}" }
                td { "{rec.user_id}" }
                td { "{rec.name}" }
                td { "{rec.job_type:?}" }
                td { "{rec.job_status:?}" }
                td { "{rec.start_time}" }
                td { "{rec.end_time}" }
                td { "{rec.comment}" }
                td {
                    Button {
                        onclick: move |_| async move {
                            match delete_job(rec.id).await {
                                Ok(_) => {
                                    msg.write().text.clear();
                                    jobs_list.restart();
                                }
                                Err(e) => msg.write().text = format!("{}", e),
                            }
                        },
                        text: "Delete",
                    }
                }
            }
        }
    }
}
