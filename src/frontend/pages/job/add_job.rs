use dioxus::prelude::*;
use std::collections::HashSet;

use crate::backend::api::api_job::new_backup;
use crate::frontend::{
    collections::{file_view::FileViewer, message::Message},
    elements::button::Button,
    modules::{
        modal::{MODAL_JOB_ID, Modal},
        tab::Tab,
    },
    pages::job::backup_job_form::BackupJobForm,
};
use crate::shared::{
    level::Level,
    models::database::model_job::{JobType, RecordJob},
};

#[component]
pub fn AddJob() -> Element {
    let mut modal_message: Signal<String> = use_signal(|| String::default());
    let mut error_message: Signal<String> = use_signal(|| String::default());
    let mut success_message: Signal<String> = use_signal(|| String::default());
    let selected_files: Signal<HashSet<String>> = use_signal(|| HashSet::new());
    let new_job: Signal<RecordJob> = use_signal(|| RecordJob::blank(JobType::Backup));

    let tab_options = rsx! {
        BackupJobForm { job: new_job }
    };

    let tab_files = rsx! {
        FileViewer { selected_files }
        hr {}
        for f in selected_files() {
            span { "{f}" }
            br {}
        }
    };

    let submit = move |_| async move {
        if new_job().name.is_empty() {
            modal_message.set("Name cannot be empty".to_string());
        } else if selected_files().is_empty() {
            modal_message.set("No files selected".to_string());
        } else {
            match new_backup(new_job(), selected_files()).await {
                Ok(_) => success_message.set("Added".to_string()),
                Err(e) => error_message.set(format!("{}", e)),
            }
        }
    };

    rsx! {
        Modal { id: MODAL_JOB_ID, level: Level::Error, message: modal_message }
        Tab {
            tab_names: vec!["Options".to_string(), "Files".to_string(), "Destination".to_string()],
            tab_contents: vec![
                tab_options,
                tab_files,
                rsx! {
                    p { "Implement virtual file viewer" }
                },
            ],
        }
        hr {}
        Message { level: Level::Error, text: error_message() }
        Message { level: Level::Success, text: success_message() }
        Button { r#type: "button", onclick: submit, text: "Add" }
    }
}
