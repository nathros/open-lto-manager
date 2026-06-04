use dioxus::prelude::*;

use crate::route::Route;

#[component]
pub fn Sandpit() -> Element {
    rsx! {
        div {
            Link { to: Route::SandpitShowcase {}, "UI Showcase" }
            hr {}

            span { "UI Elements" }
            br {}
            Link { to: Route::SandpitButton {}, "Button" }
            hr {}
            span { "UI Modules" }
            br {}
            Link { to: Route::SandpitModal {}, "Modal" }
            hr {}
            span { "UI Collections" }
            br {}
            Link { to: Route::SandpitMessage {}, "Message" }
            hr {}
        }
    }
}
