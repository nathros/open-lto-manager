use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::backend::database::tables::table_manufacturer::TableManufacturer;
use crate::shared::models::database::model_manufacturer::RecordManufacturer;

#[get("/api/manufacturer")]
pub async fn list_manu() -> Result<Vec<RecordManufacturer>> {
    use crate::backend::database::db::DB;

    DB.with(|db| match TableManufacturer::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}
