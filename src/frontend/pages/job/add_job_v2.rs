use dioxus::prelude::*;

use crate::{
    frontend::{collections::file_view::file_view_main::FileViewer, css::Css},
    static_concat,
};

#[component]
pub fn AddJob2() -> Element {
    rsx! {
        div { class: static_concat!(Css::FLEX_ROW, Css::FILE_VIEWER_PAIR),
            FileViewer { is_virtual: false }
            FileViewer { is_virtual: true }
        }
    }
}
