use dioxus::prelude::*;

use crate::backend::api::api_sessions::list_sessions;

#[component]
pub fn Sessions() -> Element {
    let sessions = use_loader(list_sessions)?;

    rsx! {
        table {
            tr {
                th { "UUID" }
                th { "Username" }
                th { "Expiry" }
            }
            for s in sessions() {
                tr {
                    td { "{s.uuid}" }
                    td { "{s.username}" }
                    td { "{s.expiry}" }
                }
            }
        }
    }
}
