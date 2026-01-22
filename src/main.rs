use backend::init::AppState;
use dioxus::prelude::*;

mod backend;
mod frontend;
mod route;

use crate::route::Route;

fn main() {
    #[cfg(feature = "server")]
    {
        use backend::init::init_backend;

        dioxus::LaunchBuilder::new()
            .with_context(init_backend())
            .launch(App);
    }
}

#[component]
fn App() -> Element {
    let app_state = use_context::<AppState>();

    rsx! {
        //document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        if app_state.critical_error {
            p { "Failed to start app" }
            for error_message in app_state.error_list.clone() {
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
