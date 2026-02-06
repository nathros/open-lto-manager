use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Show {}, "Show" }
            Link { to: Route::Test {}, "Test" }
            Link { to: Route::Tape { id: (0) }, "[Add/Edit Tape]" }
            Link { to: Route::AddJob { }, "[Add Job]" }
            Link { style: "margin-left:auto", to: Route::DBMan {}, "man" }
            Link { to: Route::DBType {}, "type" }
            Link { to: Route::DBTape {}, "tape" }
        }
        hr {  }

        Outlet::<Route> {}
    }
}
