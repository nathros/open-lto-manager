use std::collections::HashSet;

use crate::frontend::collections::file_view::FileViewer;
use dioxus::prelude::*;

#[component]
pub fn AddJob() -> Element {
    let selected_files: Signal<HashSet<String>> = use_signal(|| HashSet::new());

    rsx! {
        FileViewer { selected_files }
        hr {}
        for f in selected_files() {
            span { "{f}" }
            br {}
        }
    }
}
