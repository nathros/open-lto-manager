use dioxus::prelude::*;

#[component]
pub fn CodeBlock(
    #[props(optional)] language: Option<&'static str>,
    #[props(optional)] header: Option<&'static str>,
    code: String,
) -> Element {
    // TODO styles
    rsx! {
        div { style: "background-color:grey",
            if let Some(lan) = language {
                span { "{lan}" }
            }
            if let Some(head) = header {
                span { "{head}" }
            }
            pre { {code} }
        }
    }
}
