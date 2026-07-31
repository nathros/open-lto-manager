use dioxus::prelude::*;

use crate::frontend::collections::floating_debug::FloatingDebug;

#[component]
pub fn SandpitFloating() -> Element {
    rsx! {
        FloatingDebug {
            p { "Hold left click and drag to move" }
            hr {}
            p { "beta" }
        }
    }
}
