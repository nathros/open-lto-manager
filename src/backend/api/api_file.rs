use dioxus::prelude::*;

use crate::shared::models::database::model_file::RecordFile;

#[get("/api/file/all")]
pub async fn list_files() -> Result<Vec<RecordFile>> {
    use crate::backend::database::{db::DB, tables::table_file::TableFile};

    #[cfg(feature = "slow_server")]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    DB.with(|db| match TableFile::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}
