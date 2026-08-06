use dioxus::prelude::*;

use crate::frontend::{
    elements::button::{Button, LinkButton},
    icons::Icons,
};

#[component]
pub fn SandpitButton() -> Element {
    rsx! {
        Button {
            onclick: move |_evt: MouseEvent| async move {
                let _ = document::eval("alert('Standard test button clicked')").await;
            },
            text: "[Standard] Test Button",
        }
        span { " " }
        Button {
            icon: Icons::BUG,
            onclick: move |_evt: MouseEvent| async move {
                let _ = document::eval("alert('Standard test button clicked')").await;
            },
            text: "[Standard] Test Icon Button",
        }
        span { " " }
        LinkButton { to: "#".into(), text: "[Standard] Test Link Button" }
        span { " " }
        Button {
            primary: true,
            onclick: move |_evt: MouseEvent| async move {
                let _ = document::eval("alert('Primary test button clicked')").await;
            },
            text: "[Primary] Test Button",
        }
        span { " " }
        Button {
            icon: Icons::BUG,
            primary: true,
            onclick: move |_evt: MouseEvent| async move {
                let _ = document::eval("alert('Primary test button clicked')").await;
            },
            text: "[Primary] Test Icon Button",
        }
        span { " " }
        LinkButton {
            primary: true,
            to: "#".into(),
            text: "[Standard] Test Link Button",
        }
    }
}
