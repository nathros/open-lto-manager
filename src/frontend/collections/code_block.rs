use dioxus::prelude::*;

use crate::frontend::{css::Css, js::js_copy_to_clipboard};

#[component]
pub fn CodeBlock(#[props(optional)] header: Option<&'static str>, code: String) -> Element {
    rsx! {
        div { class: Css::CODE_BLOCK,
            div {
                if let Some(head) = header {
                    span { "{head}" }
                }
                button {
                    onclick: {
                        move |_| {
                            js_copy_to_clipboard(code.as_str());
                        }
                    },
                }
            }
            pre { {code.clone()} }
        }
    }
}
