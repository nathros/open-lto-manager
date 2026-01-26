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
        return Ok(RecordTape::default());
    }

    DB.with(|db| match TableTape::get(db, id) {
        Ok(record) => Ok(record),
        Err(e) => Err(e)?,
    })
}

#[put("/api/tape")]
pub async fn api_add_tape(tape: RecordTape) -> Result<bool> {
    use crate::backend::database::{
        db::DB,
        tables::{table::Table, table_tape::TableTape},
    };

    DB.with(|db| match TableTape::insert_record(db, &tape) {
        Ok(record) => Ok(record > 1),
        Err(e) => Err(e)?,
    })
}

#[delete("/api/tape/{id}")]
pub async fn api_del_tape(id: i64) -> Result<bool> {
    use crate::backend::database::{
        db::DB,
        tables::{table::Table, table_tape::TableTape},
    };

    DB.with(|db| match TableTape::delete_record(db, id) {
        Ok(record) => Ok(record > 1),
        Err(e) => Err(e)?,
    })
}
