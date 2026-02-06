use crate::frontend::collections::file_view::FileViewer;
use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
pub fn AddJob() -> Element {
    let selected_files: Signal<Vec<PathBuf>> = use_signal(|| vec![]);

    rsx! {
        SuspenseBoundary {
            fallback: |suspense_context: SuspenseContext| {
                suspense_context.is_suspended();
                rsx! {
                    p { "loading" }
                }
            },
            FileViewer { selected_files }
        }

    }
}
