use std::collections::HashSet;

use crate::frontend::collections::file_view::FileViewer;
use crate::frontend::modules::modal::Modal;
use crate::frontend::modules::tab::Tab;
use crate::frontend::pages::job::backup_job_form::BackupJobForm;
use crate::shared::level::Level;
use crate::shared::models::database::model_job::{JobType, RecordJob};
use dioxus::prelude::*;

#[component]
pub fn AddJob() -> Element {
    let mut modal_message: Signal<String> = use_signal(|| String::default());
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
        info!("Call");
        if selected_files().is_empty() {
            modal_message.set("No files selected".to_string());
            info!("Call1 empty");
        }
    };

    rsx! {
        Modal {
            id: "modal-b_job".to_string(),
            level: Level::Error,
            message: modal_message,
        }
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
        button { r#type: "button", onclick: submit, "Add" }
    }
}
