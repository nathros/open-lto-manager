use dioxus::{fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_init::app_state,
    either,
    frontend::{
        assets::APP_NAME,
        collections::{
            code_block::CodeBlock,
            message::{Message, MessageDetails},
        },
        components::{
            card::{Card, CardOverview, CardOverviewStatus},
            main_body::MainBody,
        },
        css::Css,
        elements::{
            heading::{H2, H4},
            icon::Icon,
        },
        icons::Icons,
        level::Level,
        modules::accordion::AccordionExtended,
    },
    shared::models::{app_state::AppState, database::user::model_user::RecordUserConfig},
    static_concat,
};

#[component]
pub fn Diagnostics() -> Element {
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
        CardOverview { title: "Overview".to_string(),
            CardOverviewStatus {
                class: Icons::SUCCESS,
                title: "Passes",
                count: app_state().pass_count,
            }
            CardOverviewStatus {
                class: Icons::WARNING,
                title: "Warnings",
                count: app_state().warn_count,
            }
            CardOverviewStatus {
                class: Icons::ERROR,
                title: "Alerts",
                count: app_state().err_count,
            }
        }

        div { class: static_concat!(Css::FLEX_COL, Css::FLEX_ALIGN_LEFT),
            Card { top_padding: false,
                H2 { margin: true, "System Info" }

                div { class: Css::FLEX_ROW,
                    div { class: static_concat!(Css::FLEX_COL, Css::FLEX_ALIGN_LEFT),
                        b { "OS:" }
                        b { "Platform:" }
                        b { "CPU Architecture:" }
                        b { "System User:" }
                    }
                    div { class: static_concat!(Css::FLEX_COL, Css::FLEX_ALIGN_LEFT),
                        span { "{app_state().distro}" }
                        span { "{app_state().platform}" }
                        span { "{app_state().cpu_arch}" }
                        span { "{app_state().user_name.unwrap_or_default()}" }
                    }
                }
            }
            if let Some(usr) = app_state().user_name {
                Card { top_padding: false,
                    H2 { margin: true, "Checklist" }
                    div { class: Css::FLEX_COL,
                        AccordionExtended {
                            header: rsx! {
                                Icon {
                                    icon: either!(app_state().user_part_of_group, Icons::SUCCESS, Icons::ERROR),
                                    size: Css::MD,
                                }
                                span {
                                    "System user: "
                                    b { "'{usr}'" }
                                    span { style: "text-decoration: underline", {either!(app_state().user_part_of_group, "", " not")} }
                                    " found in group: "
                                    b { "'{app_state().group}'" }
                                }
                            },
                            div {
                                p {
                                    "In order to access tape devices {APP_NAME} needs access to the following devices:"
                                }
                                H4 { "/dev/st{{X}} and /dev/nst{{X}}" }
                                p {
                                    "Permissions to these resources as part of the '{app_state().group}' user group"
                                }
                                p {
                                    "Make sure the os user: '{usr}' is part of the '{app_state().group}'"
                                }
                                CodeBlock {
                                    language: "Fix issue:",
                                    code: format!("sudo usermod -a -G {} {}", app_state().group, usr),
                                }
                                Message {
                                    small: true,
                                    details: MessageDetails {
                                        level: Level::Info,
                                        text: "Logout/in and app restart needed to take effect".to_string(),
                                    },
                                }
                            }
                        }
                        AccordionExtended {
                            header: rsx! {
                                Icon {
                                    icon: either!(app_state().ltfs_installed, Icons::SUCCESS, Icons::ERROR),
                                    size: Css::MD,
                                }
                                span {
                                    "LTFS driver "
                                    span { style: "text-decoration: underline", {either!(app_state().ltfs_installed, "", " not")} }
                                    " found"
                                }
                            },
                            p {
                                "Without a valid driver LTFS operations will not be available, tar fallback can be used"
                            }
                        }
                        if let Some(ltfs_v) = app_state().ltfs_version
                            && let Some(ltfs_l) = app_state().ltfs_version_latest
                        {
                            if app_state().ltfs_latest_is_newer {
                                AccordionExtended {
                                    header: rsx! {
                                        Icon { icon: Icons::WARNING, size: Css::MD }
                                        span { "Newer LTFS driver is avilable" }
                                    },
                                    p { "Current version: {ltfs_v}, latest version: {ltfs_l}" }
                                }
                            } else {
                                AccordionExtended {
                                    header: rsx! {
                                        Icon { icon: Icons::SUCCESS, size: Css::MD }
                                        span { "Using latest LTFS driver" }
                                    },
                                    p { "Using current latest version: {ltfs_v}" }
                                }
                            }
                        }
                    }
                }
            }
            LiveEdit { app_state }
        }
    }
}

#[cfg(debug_assertions)]
#[component]
fn LiveEdit(app_state: Loader<AppState>) -> Element {
    let style = "label{width:9rem}";
    rsx! {
        Card {
            style { dangerous_inner_html: style }
            H2 { "Debug Editor" }
            div { class: Css::FLEX_ROW, style: "align-items:normal",
                div { class: static_concat!(Css::FLEX_COL, Css::FLEX_ALIGN_LEFT),
                    div { class: Css::FLEX_ROW,
                        label { "user_name:" }
                        input {
                            value: app_state().user_name,
                            oninput: move |evt: Event<FormData>| {
                                if evt.value().is_empty() {
                                    app_state.write().user_name = None;
                                } else {
                                    app_state.write().user_name = Some(evt.value());
                                }
                            },
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "group:" }
                        input {
                            value: app_state().group,
                            oninput: move |evt: Event<FormData>| {
                                app_state.write().group = evt.value();
                            },
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "user_part_of_group:" }
                        input {
                            r#type: "checkbox",
                            checked: app_state().user_part_of_group,
                            oninput: move |evt: Event<FormData>| {
                                app_state.write().user_part_of_group = evt.checked();
                            },
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "ltfs_installed:" }
                        input {
                            r#type: "checkbox",
                            checked: app_state().ltfs_installed,
                            oninput: move |evt: Event<FormData>| {
                                app_state.write().ltfs_installed = evt.checked();
                            },
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "ltfs_version:" }
                        input {
                            value: app_state().ltfs_version,
                            oninput: move |evt: Event<FormData>| {
                                if evt.value().is_empty() {
                                    app_state.write().ltfs_version = None;
                                } else {
                                    app_state.write().ltfs_version = Some(evt.value());
                                }
                            },
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "ltfs_version_latest:" }
                        input {
                            value: app_state().ltfs_version_latest,
                            oninput: move |evt: Event<FormData>| {
                                if evt.value().is_empty() {
                                    app_state.write().ltfs_version_latest = None;
                                } else {
                                    app_state.write().ltfs_version_latest = Some(evt.value());
                                }
                            },
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "ltfs_latest_is_newer:" }
                        input {
                            r#type: "checkbox",
                            checked: app_state().ltfs_latest_is_newer,
                            oninput: move |evt: Event<FormData>| {
                                app_state.write().ltfs_latest_is_newer = evt.checked();
                            },
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "ltfs_error:" }
                        input {
                            value: app_state().ltfs_error,
                            oninput: move |evt: Event<FormData>| {
                                if evt.value().is_empty() {
                                    app_state.write().ltfs_error = None;
                                } else {
                                    app_state.write().ltfs_error = Some(evt.value());
                                }
                            },
                        }
                    }
                }
                div { class: static_concat!(Css::FLEX_COL, Css::FLEX_ALIGN_LEFT),
                    div { class: Css::FLEX_ROW,
                        label { "platform:" }
                        input {
                            value: app_state().platform,
                            oninput: move |evt: Event<FormData>| {
                                app_state.write().platform = evt.value();
                            },
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "distro:" }
                        input {
                            value: app_state().distro,
                            oninput: move |evt: Event<FormData>| {
                                app_state.write().distro = evt.value();
                            },
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "cpu_arch:" }
                        input {
                            value: app_state().cpu_arch,
                            oninput: move |evt: Event<FormData>| {
                                app_state.write().cpu_arch = evt.value();
                            },
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "critical_error:" }
                        input {
                            r#type: "checkbox",
                            checked: app_state().critical_error,
                            oninput: move |evt: Event<FormData>| {
                                app_state.write().critical_error = evt.checked();
                            },
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "error_list:" }
                        input {
                            value: app_state().error_list.join(","),
                            placeholder: "Example: err1,err2,err3",
                            oninput: move |evt: Event<FormData>| {
                                app_state.write().error_list.clear();
                                evt.value()
                                    .split(",")
                                    .for_each(|f| {
                                        app_state.write().error_list.push(f.to_string());
                                    });
                            },
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "pass_count:" }
                        input {
                            r#type: "number",
                            value: app_state().pass_count,
                            oninput: move |evt: Event<FormData>| {
                                app_state.write().pass_count = evt.parsed::<i32>().unwrap_or(0);
                            },
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "warn_count:" }
                        input {
                            r#type: "number",
                            value: app_state().warn_count,
                            oninput: move |evt: Event<FormData>| {
                                app_state.write().warn_count = evt.parsed::<i32>().unwrap_or(0);
                            },
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "err_count:" }
                        input {
                            r#type: "number",
                            value: app_state().err_count,
                            oninput: move |evt: Event<FormData>| {
                                app_state.write().err_count = evt.parsed::<i32>().unwrap_or(0);
                            },
                        }
                    }
                }
            }
        }
    }
}

#[cfg(not(debug_assertions))]
#[component]
fn LiveEdit(app_state: Loader<AppState>) -> Element {
    rsx! {} // Nothing for release build
}

#[component]
pub fn DiagnosticsFallback() -> Element {
    let reset = // TODO cleanup
        "body{margin:0;margin-left:var(--padding-m);margin-top:3rem}.card-overview{margin-right:0}";
    let banner = "margin:0;position:fixed;left:0;top:0;right:0;text-align:center;z-index:1;background-color:red;color:white";

    rsx! {
        style { dangerous_inner_html: reset }
        h1 { class: Css::REVERT, style: banner, "Failed to start App" }
        MainBody { user: RecordUserConfig::default(), Diagnostics {} }
    }
}
