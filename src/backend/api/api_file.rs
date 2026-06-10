use dioxus::prelude::*;

use crate::shared::models::database::file::model_file::RecordFile;

#[get("/api/file/all")]
pub async fn list_files() -> Result<Vec<RecordFile>> {
    use crate::backend::database::{db::DB, tables::file::table_file::TableFile};

    DB.with(|db| match TableFile::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}
