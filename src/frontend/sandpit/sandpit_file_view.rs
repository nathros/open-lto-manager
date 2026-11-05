use dioxus::prelude::*;

use crate::frontend::{collections::file_view::file_view_main::FileViewer, css::Css};

#[component]
pub fn SandpitFileView() -> Element {
    rsx! {
        div { class: Css::FLEX_ROW,
            FileViewer { is_virtual: true }
            FileViewer { is_virtual: false }
        }
    }
}
