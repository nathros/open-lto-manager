use crate::backend::database::models::model_manufacturer::RecordManufacturer;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::backend::database::tables::table_manufacturer::TableManufacturer;

#[get("/api/manufacturer")]
pub async fn list_manu() -> Result<Vec<RecordManufacturer>> {
    use crate::backend::database::db::DB;
    DB.with(|db| Ok(TableManufacturer::get_all(db).unwrap()))
}
