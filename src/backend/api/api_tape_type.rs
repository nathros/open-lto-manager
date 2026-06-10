use dioxus::prelude::*;

use crate::shared::models::database::model_tape_type::RecordTapeType;

#[get("/api/tape_type")]
pub async fn list_type_type() -> Result<Vec<RecordTapeType>> {
    use crate::backend::database::{db::DB, tables::table_tape_type::TableTapeType};

    DB.with(|db| match TableTapeType::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}
