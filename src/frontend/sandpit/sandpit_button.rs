use dioxus::prelude::*;

use crate::frontend::elements::button::Button;

#[component]
pub fn SandpitButton() -> Element {
    rsx! {
        Button {
            onclick: move |_| async move {
                let _ = document::eval("alert('Standard test button clicked')").await;
            },
            text: "[Standard] Test Button",
        }
        span { " " }
        Button {
            primary: true,
            onclick: move |_| async move {
                let _ = document::eval("alert('Primary test button clicked')").await;
            },
            text: "[Primary] Test Button",
        }
    }
}
