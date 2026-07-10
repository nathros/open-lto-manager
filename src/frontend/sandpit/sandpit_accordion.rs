use dioxus::prelude::*;

use crate::frontend::modules::accordion::Accordion;

#[component]
pub fn SandpitAccordion() -> Element {
    rsx! {
        Accordion { label: "Test Label".to_string(),
            p { "Test content" }
        }
    }
}
