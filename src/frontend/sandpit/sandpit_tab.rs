use dioxus::prelude::*;

use crate::frontend::{
    icons::Icons,
    modules::tab::{Tab, TabLabel},
};

#[component]
pub fn SandpitTab() -> Element {
    rsx! {
        Tab {
            labels: vec![
                TabLabel::new("Tab A", Some(Icons::BUG)),
                TabLabel::new("Tab B", Some(Icons::SEARCH)),
                TabLabel::new("Tab C", Some(Icons::USER)),
            ],
            contents: vec![
                rsx! {
                    p { "Content A" }
                },
                rsx! {
                    p { "Content B" }
                },
                rsx! {
                    p { "Content C" }
                },
            ],
        }
    }
}
