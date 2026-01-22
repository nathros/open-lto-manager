use dioxus::{fullstack::Loader, prelude::*};

use crate::backend::{
    api::api_tape_type::list_type_type, database::models::model_tape_type::RecordTapeType,
};

#[component]
pub fn DBType() -> Element {
    let tapes_list: Loader<Vec<RecordTapeType>> = use_loader(list_type_type)?;

    rsx! {
        table {
            tr {
                th { "id" }
                th { "generation" }
                th { "id_reg" }
                th { "id_worm" }
                th { "native_capacity" }
                th { "colour_reg" }
                th { "colour_hp" }
                th { "colour_worm_reg" }
                th { "colour_worm_hp" }
            }
            for rec in tapes_list.cloned() {
                tr {
                    td { "{rec.id}" }
                    td { "{rec.generation}" }
                    td { "{rec.id_reg}" }
                    td { "{rec.id_worm}" }
                    td { "{rec.native_capacity}" }
                    td { "{rec.colour_reg}" }
                    td { "{rec.colour_hp}" }
                    td { "{rec.colour_worm_reg}" }
                    td { "{rec.colour_worm_hp}" }
                }
            }
        }
    }
}
