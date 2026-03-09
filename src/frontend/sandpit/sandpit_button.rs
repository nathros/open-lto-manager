use dioxus::prelude::*;

use crate::frontend::elements::button::Button;

#[component]
pub fn SandpitButton() -> Element {
    rsx! {
        Button {
            onclick: move |_| async move {
                let _ = document::eval("alert('Test button clicked')").await;
            },
            text: "Test Button",
        }
    }
}
