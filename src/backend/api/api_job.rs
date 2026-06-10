use std::collections::HashSet;

use dioxus::prelude::*;

use crate::shared::models::database::job::model_job::RecordJob;

#[get("/api/job/all")]
pub async fn list_jobs() -> Result<Vec<RecordJob>> {
    use crate::backend::database::{db::DB, tables::job::table_job::TableJob};

    DB.with(|db| match TableJob::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}

#[post("/api/job/new_backup")]
pub async fn new_backup(new_job: RecordJob, files: HashSet<String>) -> Result<bool> {
    use crate::{
        backend::database::{
            db::DB,
            tables::{
                job::table_job::TableJob,
                job_metadata::table_job_metadata::TableJobMetadata,
                table::{RecordDelete, RecordInsert, RecordInsertBatch},
            },
        },
        shared::models::database::job_metadata::model_job_metadata::{
            JobMetadataKey, RecordJobMetadata,
        },
    };

    DB.with(|db| {
        let job_id = match TableJob::insert(db, &new_job) {
            Ok(id) => id,
            Err(e) => return Err(e)?,
        };

        // Now add metadata
        let mut files_meta = Vec::with_capacity(files.len());

        for (index, file_name) in files.iter().enumerate() {
            files_meta.push(RecordJobMetadata {
                id: 0,
                job_id,
                key: JobMetadataKey::FilePhysical,
                index: index as i64,
                value: file_name.to_owned(),
            });
        }

        match TableJobMetadata::insert_batch(db, &files_meta) {
            Ok(_) => Ok(true),
            Err(e) => {
                error!("Failed to insert JobMetadata batch {} for {:?}", e, new_job);
                // On error to insert clear up job
                match TableJob::delete(db, job_id) {
                    Ok(_) => Err(e)?,
                    Err(e_inner) => Err(e_inner)?,
                }
            }
        }
    })
}

#[delete("/api/job")]
pub async fn delete_job(job_id: i64) -> Result<usize> {
    use crate::backend::database::{
        db::DB,
        tables::{job::table_job::TableJob, table::RecordDelete},
    };

    DB.with(|db| match TableJob::delete(db, job_id) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}
