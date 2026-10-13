use dioxus::prelude::*;

#[component]
pub fn Loading() -> Element {
    rsx! {
        h2 { "Loading" } // TODO make spinner
    }
}
