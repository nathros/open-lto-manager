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

#[get("/api/tape/{id}")]
pub async fn api_get_tape(id: i64) -> Result<RecordTape> {
    use crate::backend::database::{
        db::DB,
        tables::{table::Table, table_tape::TableTape},
    };

    if id == 0 {
        return Ok(RecordTape::blank());
    }

    DB.with(|db| match TableTape::get(db, id) {
        Ok(record) => Ok(record),
        Err(e) => Err(e)?,
    })
}
