#![cfg_attr(not(test), forbid(unsafe_code))] // Disable unsafe{} but allow in tests
//#![deny(warnings)] // Do not allow warnings

use dioxus::prelude::*;
use frontend::{
    assets::{CSS_ASSETS, FAVICON, JS_ASSETS},
    pages::system::diagnostics::DiagnosticsFallback,
};

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

        dioxus::serve(|| async {
            use backend::auth::SessionId;
            Ok(dioxus::server::router(App).layer(axum::middleware::from_fn(SessionId::layer)))
        });
    }

    #[cfg(not(feature = "server"))]
    dioxus::LaunchBuilder::new().launch(App);
}

#[component]
fn App() -> Element {
    let app_state = use_loader(app_state)?;

    rsx! {
        document::Link { rel: "icon", r#type: "image/svg+xml", href: FAVICON }
        for asset in JS_ASSETS.iter() {
            script { r#type: "text/javascript", src: *asset }
        }
        for asset in CSS_ASSETS.iter() {
            document::Link { rel: "stylesheet", href: *asset }
        }

        if app_state().critical_error {
            DiagnosticsFallback {}
        } else {
            Router::<Route> {}
        }
    }
}
