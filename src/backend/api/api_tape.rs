use dioxus::prelude::*;

use crate::shared::models::database::tape::model_tape::RecordTape;

#[get("/api/tape")]
pub async fn list_tape() -> Result<Vec<RecordTape>> {
    use crate::backend::database::{db::DB, tables::tape::table_tape::TableTape};

    DB.with(|db| match TableTape::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}

#[get("/api/tape/{id}")]
pub async fn api_get_tape(id: i64) -> Result<RecordTape> {
    use crate::backend::database::{
        db::DB,
        tables::{table::RecordRead, tape::table_tape::TableTape},
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
        tables::{table::RecordInsert, tape::table_tape::TableTape},
    };

    DB.with(|db| match TableTape::insert(db, &tape) {
        Ok(record) => Ok(record > 1),
        Err(e) => Err(e)?,
    })
}

#[delete("/api/tape/{id}")]
pub async fn api_del_tape(id: i64) -> Result<bool> {
    use crate::backend::database::{
        db::DB,
        tables::{table::RecordDelete, tape::table_tape::TableTape},
    };

    DB.with(|db| match TableTape::delete(db, id) {
        Ok(record) => Ok(record > 1),
        Err(e) => Err(e)?,
    })
}

#[get("/api/tape/check/barcode?barcode")]
pub async fn api_tape_barcode_exists(barcode: String) -> Result<bool> {
    use crate::backend::database::{db::DB, tables::tape::table_tape::TableTape};

    DB.with(|db| match TableTape::barcode_exists(db, barcode) {
        Ok(found) => Ok(found),
        Err(e) => Err(e)?,
    })
}

#[get("/api/tape/check/serial?serial")]
pub async fn api_tape_serial_exists(serial: String) -> Result<bool> {
    use crate::backend::database::{db::DB, tables::tape::table_tape::TableTape};

    DB.with(|db| match TableTape::serial_exists(db, serial) {
        Ok(found) => Ok(found),
        Err(e) => Err(e)?,
    })
}
