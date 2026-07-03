use dioxus::prelude::*;

use crate::frontend::components::card::Card;

#[component]
pub fn ViewLibrary() -> Element {
    rsx! {
        Card { top_padding: false,
            h3 { "Library" }
        }
    }
}
