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

        b { "Username Error: " }
        span { "{app_state().user_name_error.unwrap_or(\"\".to_string())}" }
        br {}

        b { "Tape Group: " }
        span { "{app_state().part_tape_group}" }
        br {}

        b { "LTFS Installed: " }
        span { "{app_state().ltfs_installed}" }
        br {}

        b { "LTFS Error: " }
        span { "{app_state().ltfs_error.unwrap_or(\"\".to_string())}" }
        br {}

        b { "MT Installed: " }
        span { "{app_state().mt_installed}" }
        br {}

        b { "Platform: " }
        span { "{app_state().platform}" }
        br {}

        b { "Distro: " }
        span { "{app_state().distro}" }
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
