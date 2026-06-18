use dioxus::{fullstack::FullstackContext, prelude::*};
use std::mem::discriminant;

#[cfg(all(feature = "auto_login", debug_assertions))]
use crate::backend::api::api_login::api_login_bypass;

use crate::{
    Route,
    backend::api::{api_login::api_current_user, api_logout::api_logout, api_user::update_user},
    either,
    frontend::{
        assets::APP_NAME,
        collections::message::{Message, MessageDetails},
        components::colour_mode::ColourModeHidden,
        css::Css,
        elements::input::InputType,
        icons::Icons,
        level::Level,
        pages::login::login_user::LoginUser,
    },
    shared::models::database::user::model_user::{
        ACCENT_BLUE, ACCENT_GREEN, ACCENT_RED, ACCENT_STANDARD, ColourMode,
    },
    static_concat,
};

fn route_eq<T>(a: &T, b: &T) -> bool {
    discriminant(a) == discriminant(b)
}

#[component]
pub fn Header() -> Element {
    let mut current_user = use_loader(api_current_user)?;

    #[cfg(all(feature = "auto_login", debug_assertions))]
    {
        let mut skip = use_signal(|| false);
        if !skip() && current_user().is_none() {
            spawn(async move {
                // TODO fix called twice
                if api_login_bypass().await.is_ok() {
                    skip.set(true);
                    current_user.restart();
                }
            });
        }
    }

    let route: Route = use_route();

    #[cfg(debug_assertions)]
    let debug_build = true; // Show Sandpit for debug build
    #[cfg(not(debug_assertions))]
    let debug_build = false;

    let error_handler = move |err: ErrorContext| {
        let mut msg = MessageDetails::default();
        if let Some(e) = err.error() {
            let http_error = FullstackContext::commit_error_status(e);
            match http_error.status {
                StatusCode::NOT_FOUND => msg.text = "404 - Page not found".to_string(),
                StatusCode::UNAUTHORIZED => {
                    msg.text = "401 - Unauthorized".to_string();
                    msg.level = Level::Warning;
                }
                StatusCode::INTERNAL_SERVER_ERROR => {
                    msg.text = "500 - Internal Server Error".to_string();
                }
                _ => msg.text = "An unknown error occurred".to_string(),
            }
        }
        //let cloned_errors = err.clone(); // Clear on load
        //use_effect(move || {
        //    cloned_errors.clear_errors();
        //    error!("clear");
        //});
        rsx! {
            Message { details: msg }
            p { "-- Refresh needed --" }
            button {
                onclick: move |_| {
                    err.clear_errors();
                },
                "Retry"
            }
        }
    };

    let mut show_menu = use_signal(|| false);
    let mut show_theme = use_signal(|| false);
    let mut show_accent = use_signal(|| false);
    let mut close = move |evt: Event<MouseData>| {
        evt.stop_propagation();
        show_menu.set(false);
        show_theme.set(false);
        show_accent.set(false);
    };
    let change_theme = move |theme: ColourMode| async move {
        if let Some(mut user) = current_user() {
            user.system_theme = theme;
            if update_user(user.clone()).await.is_ok() {
                current_user.set(Some(user));
            }
        }
    };
    let change_accent = move |colour: String| async move {
        if let Some(mut user) = current_user() {
            user.accent_colour = colour.clone().to_string();
            if update_user(user.clone()).await.is_ok() {
                current_user.set(Some(user));
            }
        }
    };

    rsx! {
        if let user = current_user().unwrap_or_default()
            && let icon_theme = format!("{:?}", user.icon_theme).to_lowercase()
        {
            div { style: "--icon-theme:{icon_theme}; --colour-accent:{user.accent_colour}",
                ColourModeHidden { theme: user.system_theme }
                if current_user().is_some() {
                    aside { class: Css::MAIN_ASIDE,
                        div {
                            Link { to: Route::Home {}, "Home" }
                            Link {
                                style: if route_eq(&route, &Route::Show {}) { "background-color: cyan" } else { "background-color: white" },
                                to: Route::Show {},
                                "Show"
                            }
                            Link {
                                style: if route_eq(&route, &Route::Test {}) { "background-color: cyan" } else { "background-color: white" },
                                to: Route::Test {},
                                "Test"
                            }
                            Link {
                                style: if route_eq(&route, &Route::Tape { id: 0 }) { "background-color: cyan" } else { "background-color: white" },
                                to: Route::Tape { id: (0) },
                                "[Add/Edit Tape]"
                            }
                            Link {
                                style: if route_eq(&route, &Route::AddJob {}) { "background-color: cyan" } else { "background-color: white" },
                                to: Route::AddJob {},
                                "[Add Job]"
                            }
                            Link { to: Route::ShowDevices {}, "Devices" }
                            Link { to: Route::Sessions {}, "Sessions" }
                            Link { to: Route::LoginUser {}, "Login" }
                            hr { style: "width:100%" }
                            Link { to: Route::DBMan {}, "Manufacturer" }
                            Link { to: Route::DBType {}, "Type" }
                            Link { to: Route::DBUser {}, "User" }
                            Link { to: Route::DBFile {}, "File" }
                            Link { to: Route::DBJob {}, "Job" }
                            Link { to: Route::DBJobMetaData {},
                                {}
                                "Job-Meta"
                            }
                            Link { to: Route::DBTape {}, "Tape" }
                            Link { to: Route::ShowAppState {}, "AppState" }
                            if debug_build {
                                Link { to: Route::Sandpit {}, "Sandpit" }
                                Link { to: Route::ShowDev {}, "Dev" }
                            }
                        }
                    }
                    header { class: Css::MAIN_HEADER,
                        div { class: Css::MAIN_HEADER_LOGO, "{APP_NAME}" }
                        div { class: static_concat!(Css::ICON, Css::SM, Icons::NOTIFICATION) }
                        div { class: static_concat!(Css::ICON, Css::SM, Icons::INFO) }
                        div {
                            class: [Css::HEADER_DROPDOWN, either!(show_menu(), Css::SHOW, "")].concat(),
                            onclick: move |evt: Event<MouseData>| {
                                evt.stop_propagation();
                                show_menu.set(!show_menu());
                            },
                            div {
                                class: [Css::SCREEN_FILL, either!(show_menu(), Css::SHOW, "")].concat(),
                                onclick: close,
                            }
                            span { class: static_concat!(Css::ICON, Icons::USER) }
                            div {
                                class: Css::HEADER_DROPDOWN_CONTENT,
                                onclick: move |evt: Event<MouseData>| {
                                    evt.stop_propagation();
                                },
                                div { class: static_concat!(Css::ICON_LIST_ITEM, Css::HEADER_USER),
                                    "{user.username}"
                                }
                                Link {
                                    class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW),
                                    onclick: close,
                                    to: Route::DBUser {},
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::USER) }
                                    span { "Account" }
                                }
                                div {
                                    class: [Css::ICON_LIST_ITEM, Css::FLEX_ROW, either!(show_accent(), Css::SELECTED, "")]
                                        .concat(),
                                    onclick: move |evt: Event<MouseData>| {
                                        evt.stop_propagation();
                                        show_accent.set(!show_accent());
                                        show_theme.set(false);
                                    },
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::PALETTE) }
                                    span { "Accent" }
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::CHEVRON_RIGHT, Css::FLOAT_RIGHT) }
                                    div {
                                        class: [
                                            Css::HEADER_DROPDOWN_CONTENT,
                                            Css::HEADER_DROPDOWN_NESTED,
                                            either!(show_accent(), Css::SHOW, ""),
                                        ]
                                            .concat(),
                                        div {
                                            class: [
                                                Css::ICON_LIST_ITEM,
                                                Css::FLEX_ROW,
                                                either!(user.accent_colour == ACCENT_STANDARD, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_accent(ACCENT_STANDARD.into()).await;
                                                close(evt);
                                            },
                                            span {
                                                style: static_concat!("background-color:", ACCENT_STANDARD),
                                                class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL),
                                            }
                                            span { "Standard" }
                                        }
                                        div {
                                            class: [
                                                Css::ICON_LIST_ITEM,
                                                Css::FLEX_ROW,
                                                either!(user.accent_colour == ACCENT_RED, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_accent(ACCENT_RED.into()).await;
                                                close(evt);
                                            },
                                            span {
                                                style: static_concat!("background-color:", ACCENT_RED),
                                                class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL),
                                            }
                                            span { "Red" }
                                        }
                                        div {
                                            class: [
                                                Css::ICON_LIST_ITEM,
                                                Css::FLEX_ROW,
                                                either!(user.accent_colour == ACCENT_GREEN, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_accent(ACCENT_GREEN.into()).await;
                                                close(evt);
                                            },
                                            span {
                                                style: static_concat!("background-color:", ACCENT_GREEN),
                                                class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL),
                                            }
                                            span { "Green" }
                                        }
                                        div {
                                            class: [
                                                Css::ICON_LIST_ITEM,
                                                Css::FLEX_ROW,
                                                either!(user.accent_colour == ACCENT_BLUE, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_accent(ACCENT_BLUE.into()).await;
                                                close(evt);
                                            },
                                            span {
                                                style: static_concat!("background-color:", ACCENT_BLUE),
                                                class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL),
                                            }
                                            span { "Blue" }
                                        }
                                        label {
                                            class: [
                                                Css::ICON_LIST_ITEM,
                                                Css::FLEX_ROW,
                                                either!(
                                                    user.accent_colour == ACCENT_STANDARD ||
                                                    user.accent_colour == ACCENT_RED ||
                                                    user.accent_colour == ACCENT_GREEN ||
                                                    user.accent_colour == ACCENT_BLUE,
                                                    "",
                                                    Css::SELECTED
                                                ),
                                            ]
                                                .concat(),
                                            r#for: Css::ID_ACCENT_PICKER,
                                            onclick: move |evt: Event<MouseData>| {
                                                evt.stop_propagation();
                                            },
                                            span { class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL, Css::RAINBOW) }
                                            span { "Custom" }
                                            input {
                                                id: Css::ID_ACCENT_PICKER,
                                                r#type: InputType::Colour.to_string(),
                                                oninput: move |evt| async move {
                                                    evt.stop_propagation();
                                                    change_accent(evt.value()).await;
                                                },
                                                value: user.accent_colour.clone(),
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: [Css::ICON_LIST_ITEM, Css::FLEX_ROW, either!(show_theme(), Css::SELECTED, "")]
                                        .concat(),
                                    onclick: move |evt: Event<MouseData>| {
                                        evt.stop_propagation();
                                        show_theme.set(!show_theme());
                                        show_accent.set(false);
                                    },
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::FILL_HALF) }
                                    span { "Theme" }
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::CHEVRON_RIGHT, Css::FLOAT_RIGHT) }
                                    div {
                                        class: [
                                            Css::HEADER_DROPDOWN_CONTENT,
                                            Css::HEADER_DROPDOWN_NESTED,
                                            either!(show_theme(), Css::SHOW, ""),
                                        ]
                                            .concat(),
                                        div {
                                            class: [
                                                Css::ICON_LIST_ITEM,
                                                Css::FLEX_ROW,
                                                either!(user.system_theme == ColourMode::System, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_theme(ColourMode::System).await;
                                                close(evt);
                                            },
                                            span { class: static_concat!(Css::ICON, Css::SM, Icons::SYSTEM) }
                                            span { "System" }
                                        }
                                        div {
                                            class: [
                                                Css::ICON_LIST_ITEM,
                                                Css::FLEX_ROW,
                                                either!(user.system_theme == ColourMode::Light, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_theme(ColourMode::Light).await;
                                                close(evt);
                                            },
                                            span { class: static_concat!(Css::ICON, Css::SM, Icons::LIGHT) }
                                            span { "Light" }
                                        }
                                        div {
                                            class: [
                                                Css::ICON_LIST_ITEM,
                                                Css::FLEX_ROW,
                                                either!(user.system_theme == ColourMode::Dark, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_theme(ColourMode::Dark).await;
                                                close(evt);
                                            },
                                            span { class: static_concat!(Css::ICON, Css::SM, Icons::DARK) }
                                            span { "Dark" }
                                        }
                                    }
                                }
                                hr {}
                                Link {
                                    class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW),
                                    onclick: |_| async {
                                        let _ = api_logout().await;
                                    },
                                    to: Route::LoginUser {},
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::LOGOUT) }
                                    span { "Log out" }
                                }
                            }
                        }
                    }

                    ErrorBoundary { handle_error: error_handler, Outlet::<Route> {} }
                } else {
                    ErrorBoundary { handle_error: error_handler,
                        LoginUser {
                            success_signal: move |_| async move {
                                current_user.restart();
                            },
                        }
                    }
                }
            }
        }
    }
}
