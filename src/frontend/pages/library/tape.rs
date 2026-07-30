use dioxus::{dioxus_core, fullstack::Loader, prelude::*};

use crate::{
    backend::api::{api_manufacturer::list_manufacturer, api_tape_type::list_type_type},
    frontend::{components::card::Card, pages::library::tape_form::TapeForm},
    shared::models::database::{
        manufacturer::model_manufacturer::RecordManufacturer,
        tape_type::model_tape_type::RecordTapeType,
    },
};

#[component]
pub fn Tape(id: i64) -> Element {
    let manufactures: Loader<Vec<RecordManufacturer>> = use_loader(list_manufacturer)?;
    let types: Loader<Vec<RecordTapeType>> = use_loader(list_type_type)?;

    rsx! {
        Card {
            TapeForm { id: 0, types: types(), manufacturers: manufactures() }
        }
    }
}
