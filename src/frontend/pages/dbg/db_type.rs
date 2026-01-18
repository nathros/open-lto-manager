use dioxus::prelude::*;

use crate::backend::api::api_tape_type::list_type_type;

#[component]
pub fn DBType() -> Element {
    let tapes_list = use_loader(list_type_type)?;

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
                    th { "{rec.id}" }
                    th { "{rec.generation}" }
                    th { "{rec.id_reg}" }
                    th { "{rec.id_worm}" }
                    th { "{rec.native_capacity}" }
                    th { "{rec.colour_reg}" }
                    th { "{rec.colour_hp}" }
                    th { "{rec.colour_worm_reg}" }
                    th { "{rec.colour_worm_hp}" }
                }
            }
        }
    }
}
