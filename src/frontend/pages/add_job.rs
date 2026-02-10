use crate::frontend::collections::file_view::FileViewer;
use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
pub fn AddJob() -> Element {
    let selected_files: Signal<Vec<PathBuf>> = use_signal(|| vec![]);

    rsx! {
        FileViewer { selected_files }
    }
}
