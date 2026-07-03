use dioxus::prelude::*;

use crate::frontend::modules::tab::Tab;

#[component]
pub fn SandpitTab() -> Element {
    rsx! {
        Tab {
            labels: vec!["Tab A".to_string(), "Tab B".to_string(), "Tab C".to_string()],
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
