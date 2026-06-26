use dioxus::prelude::*;

use crate::{
    frontend::{assets::IMG_SANDPIT, css::Css, elements::button::LinkButton},
    route::Route,
    static_concat,
};

#[component]
pub fn Sandpit() -> Element {
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
        div { class: static_concat!(Css::FLEX_COL, Css::FLEX_ALIGN_LEFT),
            div {
                id: Css::ID_SAND,
                style: format!("background-image:url({})", IMG_SANDPIT),

            }
            h2 { "Sandpit: Dev Testing Area" }
            div { class: Css::CARD,
                b { "Showcase" }
                br {}
                br {}
                div { class: Css::FLEX_ROW,
                    LinkButton {
                        primary: true,
                        to: Route::SandpitShowcase {}.into(),
                        text: "UI Showcase",
                    }
                }
            }

            div { class: Css::FLEX_ROW,
                for (set_name , groups) in all_items {
                    div { class: Css::CARD,
                        b { "{set_name}" }
                        br {}
                        br {}
                        div { class: Css::FLEX_ROW,
                            for (name , link) in groups {
                                LinkButton { to: link.into(), text: name }
                            }
                        }
                    }
                }
            }
        }
    }
}
