use crate::backend::database::models::model_tape::RecordTape;
use dioxus::prelude::*;

#[get("/api/tape")]
pub async fn list_tape() -> Result<Vec<RecordTape>> {
    use crate::backend::database::{db::DB, tables::table_tape::TableTape};

    DB.with(|db| match TableTape::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}
