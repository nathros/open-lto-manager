use dioxus::prelude::*;

use crate::frontend::collections::file_view::file_view_main::FileViewer;

#[component]
pub fn AddJob2() -> Element {
    rsx! {
        FileViewer { is_virtual: true }
    }
}
