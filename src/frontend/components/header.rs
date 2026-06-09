use std::mem::discriminant;

use dioxus::fullstack::FullstackContext;
use dioxus::prelude::*;

use crate::static_concat;
use crate::{
    Route,
    frontend::{
        assets::APP_NAME, collections::message::Message, collections::message::MessageDetails,
        css::Css, icons::Icons, level::Level,
    },
    shared::models::database::model_user::RecordUser,
};

fn route_eq<T>(a: &T, b: &T) -> bool {
    discriminant(a) == discriminant(b)
}

#[component]
pub fn Header() -> Element {
    let user = RecordUser::default();
    let icon_theme = format!("{:?}", user.icon_theme).to_lowercase();
    let route: Route = use_route();

    #[cfg(debug_assertions)]
    let debug_build = true; // Show Sandpit for debug build
    #[cfg(not(debug_assertions))]
    let debug_build = false;

    rsx! {
        div { style: "--icon-theme:{icon_theme}; --colour-accent:{user.accent_colour}",
            header { class: Css::MAIN_HEADER,
                div { class: "header-logo", "{APP_NAME}" }
                div { class: static_concat!("header-icon icon-s icon ", Icons::NOTIFICATION) }
                div { class: static_concat!("header-icon icon-s icon ", Icons::INFO) }
                div { class: static_concat!("header-icon icon ", Icons::USER) }
            }
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

            ErrorBoundary {
                handle_error: move |err: ErrorContext| {
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
                },
                Outlet::<Route> {}
            }
        }
    }
}
