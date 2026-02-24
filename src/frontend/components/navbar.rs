use std::mem::discriminant;

use dioxus::prelude::*;

use crate::Route;

fn route_eq<T>(a: &T, b: &T) -> bool {
    discriminant(a) == discriminant(b)
}

#[component]
pub fn Navbar() -> Element {
    let route: Route = use_route();
    let mut multiplier = use_signal(|| String::new());

    rsx! {
        div { id: "navbar",
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
            Link { style: "margin-left:auto", to: Route::DBMan {}, "Manufacturer" }
            Link { to: Route::DBType {}, "Type" }
            Link { to: Route::DBUser {}, "User" }
            Link { to: Route::DBJob {}, "Job" }
            Link { to: Route::DBFile {}, "File" }
            Link { to: Route::DBJobMetaData {},
                {}
                "File-Meta"
            }
            Link { to: Route::DBTape {}, "Tape" }
            Link { to: Route::ShowAppState {}, "AppState" }
        }
        hr {}

        ErrorBoundary {
            handle_error: move |errors: ErrorContext| {
                //let cloned_errors = errors.clone(); // Can
                //use_effect(move || {
                //    cloned_errors.clear_errors();
                //
                //});
                rsx! {
                    p { style: "color: purple", "Unrecoverable error: {errors:?}" }
                    p { "-- Refresh needed --" }
                    button {
                        onclick: move |_| {
                            errors.clear_errors();
                            multiplier.set("mutate".to_string());
                        },
                        "Retry"
                    }
                }
            },
            Outlet::<Route> {}
        }
    }
}
