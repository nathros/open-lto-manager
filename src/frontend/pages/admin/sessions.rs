use dioxus::prelude::*;

use crate::{backend::api::api_sessions::list_sessions, frontend::components::card::Card};

#[component]
pub fn Sessions() -> Element {
    let sessions = use_loader(list_sessions)?;

    rsx! {
        Card {
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
}
