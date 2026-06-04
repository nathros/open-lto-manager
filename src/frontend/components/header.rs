use dioxus::prelude::*;

use crate::{
    Route,
    frontend::{assets::APP_NAME, components::navbar::Navbar},
    shared::models::database::model_user::RecordUser,
};

#[component]
pub fn Header() -> Element {
    let icon_theme = format!("{:?}", RecordUser::default().icon_theme).to_lowercase();

    rsx! {
        div { style: "--icon-theme:{icon_theme}",
            header {
                div { class: "header-logo", "{APP_NAME}" }
                div { class: "header-icon header-notification" }
                div { class: "header-icon header-info" }
                div { class: "header-icon header-user" }
            }
            Navbar {}
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
