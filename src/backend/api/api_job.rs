use std::collections::HashSet;

use dioxus::prelude::*;

use crate::shared::models::database::model_job::RecordJob;

#[get("/api/job/all")]
pub async fn list_jobs() -> Result<Vec<RecordJob>> {
    use crate::backend::database::{db::DB, tables::table_job::TableJob};

    #[cfg(feature = "slow_server")]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    DB.with(|db| match TableJob::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}

#[post("/api/job/new_backup")]
pub async fn new_backup(new_job: RecordJob, _files: HashSet<String>) -> Result<bool> {
    use crate::backend::database::{
        db::DB,
        tables::{table::Table, table_job::TableJob},
    };

    #[cfg(feature = "slow_server")]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    DB.with(|db| Ok(TableJob::insert_record(db, &new_job).is_ok()))
}
