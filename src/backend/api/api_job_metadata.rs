use dioxus::prelude::*;

use crate::shared::models::database::job_metadata::model_job_metadata::RecordJobMetadata;

#[get("/api/job_metadata/all")]
pub async fn list_metadata() -> Result<Vec<RecordJobMetadata>> {
    use crate::backend::database::{
        db::DB, tables::job_metadata::table_job_metadata::TableJobMetadata,
    };

    DB.with(
        |db| match TableJobMetadata::<RecordJobMetadata>::get_all(db) {
            Ok(records) => Ok(records),
            Err(e) => Err(e)?,
        },
    )
}
