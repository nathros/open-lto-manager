use std::collections::HashSet;

use dioxus::prelude::*;

use crate::frontend::collections::file_view::file_view_legacy::FileViewerV1;

#[component]
pub fn SandpitFileView() -> Element {
    let selected_files: Signal<HashSet<String>> = use_signal(|| HashSet::new());
    rsx! {
        FileViewerV1 { selected_files }
        hr {}
        for f in selected_files() {
            span { "{f}" }
            br {}
        }
    }
}
