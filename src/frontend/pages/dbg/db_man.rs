use dioxus::{fullstack::Loader, prelude::*};

use crate::backend::{
    api::api_manufacturer::list_manu, database::models::model_manufacturer::RecordManufacturer,
};

#[component]
pub fn DBMan() -> Element {
    let list_manu: Loader<Vec<RecordManufacturer>> = use_loader(list_manu)?;
    rsx! {
        table {
            tr {
                th { "id" }
                th { "name" }
            }
            for rec in list_manu.cloned() {
                tr {
                    td { "{rec.id}" }
                    td { "{rec.name}" }
                }
            }
        }
    }
}
