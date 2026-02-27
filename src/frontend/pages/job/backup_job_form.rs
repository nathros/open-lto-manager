use dioxus::prelude::*;

use crate::shared::models::database::model_job::RecordJob;

#[component]
pub fn BackupJobForm(job: Signal<RecordJob>) -> Element {
    rsx! {
        label { "Job Name:" }
        br {}
        input {
            r#type: "text",
            oninput: move |evt: Event<FormData>| { job.write().name = evt.value() },
        }
        br {}
        br {}

        label { "Comment:" }
        br {}
        textarea { oninput: move |evt: Event<FormData>| { job.write().comment = evt.value() } }

        p { "Debug: {job():?}" }
    }
}
