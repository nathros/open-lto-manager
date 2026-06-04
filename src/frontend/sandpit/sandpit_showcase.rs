use dioxus::prelude::*;

use crate::frontend::sandpit::{
    sandpit_button::SandpitButton, sandpit_message::SandpitMessage, sandpit_modal::SandpitModal,
};

#[component]
pub fn SandpitShowcase() -> Element {
    rsx! {
        span { "Button" }
        div { SandpitButton {} }
        hr {}

        span { "Modal" }
        div { SandpitModal {} }
        hr {}

        span { "Message" }
        div { SandpitMessage {} }
        hr {}
    }
}
