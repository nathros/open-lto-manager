use dioxus::prelude::*;

mod backend;
mod frontend;
mod route;

use crate::{backend::api::api_init::app_state, route::Route};

fn main() {
    #[cfg(feature = "server")]
    {
        use crate::backend::init::APP_STATE;

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
        //document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

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

//const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
//const HEADER_SVG: Asset = asset!("/assets/header.svg");
