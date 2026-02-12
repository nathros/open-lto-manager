use dioxus::prelude::*;

use crate::backend::api::api_init::app_state;

#[component]
pub fn ShowAppState() -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |_suspense_context: SuspenseContext| {
                rsx! {}
            },
            Inner {}
        }
    }
}

#[component]
fn Inner() -> Element {
    let app_state = use_loader(app_state)?;

    rsx! {
        b { "Username: " }
        span { "{app_state().user_name}" }
        br {}

        b { "Platform: " }
        span { "{app_state().platform}" }
        br {}

        b { "CPU Arch: " }
        span { "{app_state().cpu_arch}" }
        br {}

        b { "Critical error: " }
        span { "{app_state().critical_error}" }
        br {}

        b { "Errors: " }
        for err in app_state().error_list {
            span { "{err}" }
            br {}
        }

    }
}
