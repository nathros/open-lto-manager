use dioxus::prelude::*;

use crate::backend::api::api_manufacturer::list_manu;

#[component]
pub fn DBMan() -> Element {
    let list_manu = use_loader(list_manu)?;
    rsx! {
        table {
            tr {
                th { "id" }
                th { "name" }
            }
            for rec in list_manu.cloned() {
                tr {
                    th { "{rec.id}" }
                    th { "{rec.name}" }
                }
            }
        }
    }
}
