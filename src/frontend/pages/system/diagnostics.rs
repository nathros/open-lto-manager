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
            link::InlineLink,
        },
        icons::Icons,
        level::Level,
        modules::accordion::AccordionExtended,
    },
    shared::models::{
        app_state::{AppState, LTFSProvider},
        database::user::model_user::RecordUserConfig,
    },
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
    const FIX_ALL_CMD: &str = "bash <(curl -L https://raw.githubusercontent.com/nathros/open-lto-manager/main/scripts/deps-install.sh)";

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
                                    {either!(app_state().user_part_of_group, "", " not")}
                                    " found in group: "
                                    b { "'{app_state().group}'" }
                                }
                            },
                            div { class: Css::FLEX_COL,
                                span {
                                    "In order to access tape devices {APP_NAME} needs access to the following devices:"
                                }
                                H4 {
                                    div { class: static_concat!(Css::FLEX_ROW, Css::FLEX_CENTRE),
                                        span { "/dev/nst[x]" }
                                        span { "/dev/st[x]" }
                                    }
                                }
                                span {
                                    "Permissions to these resources are part of the "
                                    b { "'{app_state().group}'" }
                                    " user group"
                                    if !app_state().user_part_of_group {
                                        ", make sure the system user "
                                        b { "'{usr}'" }
                                        " is part of this group."
                                    } else {
                                        "."
                                    }
                                }
                                if !app_state().user_part_of_group {
                                    CodeBlock {
                                        header: "Fix specific issue:",
                                        code: format!("sudo usermod -a -G {} {}", app_state().group, usr),
                                    }
                                    Message {
                                        small: true,
                                        details: MessageDetails {
                                            level: Level::Info,
                                            text: "Making changes to groups requires Logout/in and app restart is needed to take effect"
                                                .to_string(),
                                        },
                                    }
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
                                    {either!(app_state().ltfs_installed, "", " not")}
                                    " found"
                                }
                            },
                            div { class: Css::FLEX_COL,
                                span {
                                    "Without a valid driver LTFS operations will not be available, tar fallback can be used. Install a LTFS driver or use the provided script to install one for you."
                                }
                                CodeBlock {
                                    header: "Proposed fix:",
                                    code: FIX_ALL_CMD.to_string(),
                                }
                                Message {
                                    small: true,
                                    details: MessageDetails {
                                        level: Level::Info,
                                        text: "App restart is needed to take effect".to_string(),
                                    },
                                }
                            }
                        }
                        if app_state().ltfs_installed
                            && let is_open_ltfs = app_state().ltfs_provider == LTFSProvider::OpenLTFS
                        {
                            AccordionExtended {
                                header: rsx! {
                                    Icon { icon: either!(is_open_ltfs, Icons::SUCCESS, Icons::INFO), size: Css::MD }
                                    span { "Detected LTFS is from provider: {app_state().ltfs_provider:?}" }
                                },
                                div {
                                    p {
                                        "The "
                                        InlineLink {
                                            target: "_blank",
                                            href: "https://github.com/LinearTapeFileSystem/ltfs",
                                            label: "OpenLTFS",
                                        }
                                        " driver is recommended as this is the only one tested/developed against, however LTFS from other providers should work as expected."
                                    }
                                }
                            }
                            if is_open_ltfs && let Some(ltfs_v) = app_state().ltfs_version
                                && let Some(ltfs_l) = app_state().ltfs_version_latest
                            {
                                if app_state().ltfs_latest_is_newer {
                                    AccordionExtended {
                                        header: rsx! {
                                            Icon { icon: Icons::WARNING, size: Css::MD }
                                            span { "Newer OpenLTFS driver is available" }
                                        },
                                        div {
                                            p {
                                                "Current version is: "
                                                b { {ltfs_v} }
                                                " the latest available version is: "
                                                b { {ltfs_l} }
                                            }
                                        }
                                    }
                                } else {
                                    AccordionExtended {
                                        header: rsx! {
                                            Icon { icon: Icons::SUCCESS, size: Css::MD }
                                            span { "Using latest OpenLTFS driver" }
                                        },
                                        div {
                                            p {
                                                "Using the latest version available: "
                                                b { {ltfs_v} }
                                            }
                                        }
                                    }
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
    const PROVIDERS: [LTFSProvider; 4] = [
        LTFSProvider::OpenLTFS,
        LTFSProvider::HP,
        LTFSProvider::IBM,
        LTFSProvider::Unknown,
    ];
    rsx! {
        Card { top_padding: false,
            style { dangerous_inner_html: style }
            H2 { margin: true, "Debug Editor" }
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
                        label { "ltfs_provider:" }
                        select {
                            onchange: move |evt: Event<FormData>| {
                                PROVIDERS
                                    .iter()
                                    .for_each(|p| {
                                        if format!("{:?}", p) == evt.value() {
                                            app_state.write().ltfs_provider = p.clone();
                                        }
                                    });
                            },
                            for option in PROVIDERS
                                .iter()
                                .map(|p| {
                                    let name = format!("{:?}", p);
                                    let is_selected = app_state().ltfs_provider == *p;
                                    rsx! {
                                        option { value: name, selected: is_selected, "{name}" }
                                    }
                                })
                            {
                                {option}
                            }
                        }
                    }
                    div { class: Css::FLEX_ROW,
                        label { "ltfs_specification:" }
                        input {
                            value: app_state().ltfs_specification,
                            oninput: move |evt: Event<FormData>| {
                                if evt.value().is_empty() {
                                    app_state.write().ltfs_specification = None;
                                } else {
                                    app_state.write().ltfs_specification = Some(evt.value());
                                }
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
