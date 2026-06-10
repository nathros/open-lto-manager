use dioxus::prelude::*;

use crate::shared::models::database::model_manufacturer::RecordManufacturer;

#[get("/api/manufacturer", _auth: crate::backend::auth::SessionId)]
pub async fn list_manu() -> Result<Vec<RecordManufacturer>> {
    use crate::backend::database::{db::DB, tables::table_manufacturer::TableManufacturer};

    DB.with(|db| match TableManufacturer::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}
