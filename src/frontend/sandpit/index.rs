use dioxus::prelude::*;

use crate::route::Route;

#[component]
pub fn Sandpit() -> Element {
    let style = "border-right: 1px solid #000; padding: 1rem";

    rsx! {
        div {
            Link { to: Route::SandpitShowcase {}, "UI Showcase" }
            hr {}

            div { class: "flex-row",
                div { style,
                    b { "UI Elements" }
                    br {}
                    Link { to: Route::SandpitButton {}, "Button" }
                }
                div { style,
                    b { "UI Modules" }
                    br {}
                    Link { to: Route::SandpitModal {}, "Modal" }
                }
                div {
                    b { "UI Collections" }
                    br {}
                    Link { to: Route::SandpitMessage {}, "Message" }
                }
            }
        }
    }
}
