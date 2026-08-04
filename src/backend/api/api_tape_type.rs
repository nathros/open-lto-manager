use dioxus::prelude::*;

use crate::shared::models::database::tape_type::model_tape_type::{
    RecordTapeType, RecordTapeTypeLabel,
};

#[get("/api/tape_type")]
pub async fn list_type_type() -> Result<Vec<RecordTapeType>> {
    use crate::backend::database::{db::DB, tables::tape_type::table_tape_type::TableTapeType};

    DB.with(|db| match TableTapeType::<RecordTapeType>::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}

#[get("/api/tape_type_labels")]
pub async fn list_type_type_labels() -> Result<Vec<RecordTapeTypeLabel>> {
    use crate::backend::database::{db::DB, tables::tape_type::table_tape_type::TableTapeType};

    DB.with(
        |db| match TableTapeType::<RecordTapeTypeLabel>::get_all(db) {
            Ok(records) => Ok(records),
            Err(e) => Err(e)?,
        },
    )
}
