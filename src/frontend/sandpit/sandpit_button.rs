use dioxus::prelude::*;

use crate::{
    frontend::elements::button::{Button, LinkButton},
    route::Route,
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
        LinkButton {
            to: Route::SandpitButton {}.into(),
            text: "[Standard] Test Link Button",
        }
        span { " " }
        Button {
            primary: true,
            onclick: move |_evt: MouseEvent| async move {
                let _ = document::eval("alert('Primary test button clicked')").await;
            },
            text: "[Primary] Test Button",
        }
        span { " " }
        LinkButton {
            primary: true,
            to: Route::SandpitButton {}.into(),
            text: "[Standard] Test Link Button",
        }
    }
}
