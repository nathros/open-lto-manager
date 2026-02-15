use dioxus::{fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_job::list_jobs,
    shared::models::database::model_job::{JobStatus, JobType, RecordJob},
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
            th {"id"}
            th {"user_id"}
            th {"name"}
            th {"job_type"}
            th {"job_status"}
            th {"start_time"}
            th {"end_time"}
            th {"comment"}
            }
            {children}
        }
    }
}

#[component]
fn Inner() -> Element {
    let jobs_list: Loader<Vec<RecordJob>> = use_loader(list_jobs)?;

    rsx! {
        for rec in jobs_list.cloned() {
            tr {
                td {"{rec.id}"}
                td {"{rec.user_id}"}
                td {"{rec.name}"}
                td {"{<JobType as Into<i64>>::into(rec.job_type)}"}
                td {"{<JobStatus as Into<i64>>::into(rec.job_status)}"}
                td {"{rec.start_time}"}
                td {"{rec.end_time}"}
                td {"{rec.comment}"}
            }
        }

    }
}
