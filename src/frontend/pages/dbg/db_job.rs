use dioxus::{fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_job::{delete_job, list_jobs},
    frontend::{collections::message::Message, elements::button::Button},
    shared::{level::Level, models::database::model_job::RecordJob},
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
    let mut message: Signal<String> = use_signal(|| String::default());

    rsx! {
        Message { level: Level::Error, text: message() }
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
                                    message.write().clear();
                                    jobs_list.restart();
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
