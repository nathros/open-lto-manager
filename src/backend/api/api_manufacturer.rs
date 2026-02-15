use dioxus::prelude::*;

use crate::shared::models::database::model_manufacturer::RecordManufacturer;

#[get("/api/manufacturer")]
pub async fn list_manu() -> Result<Vec<RecordManufacturer>> {
    use crate::backend::database::{db::DB, tables::table_manufacturer::TableManufacturer};

    #[cfg(feature = "slow_server")]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    DB.with(|db| match TableManufacturer::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}
