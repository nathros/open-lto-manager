use dioxus::prelude::*;

use crate::{frontend::elements::button::LinkButton, route::Route};

#[component]
pub fn Sandpit() -> Element {
    let style = "border-right: 1px solid #000; padding: 1rem";

    let all_items = [
        (
            "UI Elements",
            vec![
                ("Button", Route::SandpitButton {}.to_string()),
                ("Menu Item", Route::SandpitMenuItem {}.to_string()),
            ],
        ),
        (
            "UI Modules",
            vec![("Modal", Route::SandpitMessage {}.to_string())],
        ),
        (
            "UI Collections",
            vec![("Message", Route::SandpitMessage {}.to_string())],
        ),
        (
            "UI Components",
            vec![("Menu", Route::SandpitMenu {}.to_string())],
        ),
    ];

    rsx! {
        div {
            LinkButton {
                primary: true,
                to: Route::SandpitShowcase {}.into(),
                text: "UI Showcase",
            }
            br {}
            br {}
            hr {}

            div { class: "flex-row",
                for (set_name , groups) in all_items {
                    div { style,
                        b { "{set_name}" }
                        br {}
                        br {}
                        for (name , link) in groups {
                            LinkButton { to: link.into(), text: name }
                        }
                    }
                }
            }
        }
    }
}
