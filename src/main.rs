#![cfg_attr(not(test), forbid(unsafe_code))] // Disable unsafe{} but allow in tests

use dioxus::prelude::*;
use frontend::assets::{CSS_ASSETS, FAVICON};

mod backend;
#[allow(clippy::redundant_closure, irrefutable_let_patterns)]
// FIXME Dioxus use_signal falsely triggers redundant_closure
mod frontend;
mod route;
mod shared;

use crate::{backend::api::api_init::app_state, route::Route};

fn main() {
    #[cfg(feature = "server")]
    {
        use crate::backend::init::APP_STATE;

        // Initialise APP_STATE at startup
        let init_state = APP_STATE.clone();
        if init_state.critical_error {
            error!("Failure in startup");
            for error_message in init_state.error_list {
                error!("Error: {}", error_message);
            }
        }
    }

    dioxus::LaunchBuilder::new().launch(App);
}

#[component]
fn App() -> Element {
    let app_state = use_loader(app_state)?;

    rsx! {
        document::Link { rel: "icon", r#type: "image/svg+xml", href: FAVICON }
        for asset in CSS_ASSETS.iter() {
            document::Link { rel: "stylesheet", href: *asset }
        }

        if app_state().critical_error {
            p { "Failed to start app" }
            for error_message in app_state().error_list.clone() {
                p { style: "color:red", "{error_message}" }
            }
            p { " -- show logs -- // TODO " }
        } else {
            Router::<Route> {}
        }
    }
}
