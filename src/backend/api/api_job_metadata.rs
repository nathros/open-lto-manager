use dioxus::prelude::*;

use crate::shared::models::database::model_job_metadata::RecordJobMetadata;

#[get("/api/job_metadata/all")]
pub async fn list_metadata() -> Result<Vec<RecordJobMetadata>> {
    use crate::backend::database::db::DB;
    use crate::backend::database::tables::table_job_metadata::TableJobMetadata;

    DB.with(|db| match TableJobMetadata::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}
