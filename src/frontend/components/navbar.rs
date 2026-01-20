use dioxus::prelude::*;

use crate::Route;

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Show {}, "Show" }
            Link { to: Route::Test {}, "Test" }
            Link { to: Route::Tape { id: (0) }, "Tape" }
            Link { style: "margin-left:auto", to: Route::DBMan {}, "database-man" }
            Link { to: Route::DBType {}, "database-type" }
        }

        Outlet::<Route> {}
    }
}
