/*
impl RecordRead<RecordBackupJob> for TableJob<RecordBackupJob> {
    fn get(db: &Connection, record_id: i64) -> Result<RecordBackupJob, rusqlite::Error> {
        db.prepare(
            "SELECT
                id,
                user_id,
                name,
                job_type,
                job_status,
                start_time,
                end_time,
                comment
            FROM job
            WHERE id = ?1",
        )?
        .query_one([record_id], |row| TableJob::fill(row, 0))
    }
}

impl RecordInsert<RecordJob> for TableJob<RecordJob> {
    fn insert(db: &Connection, record: &RecordJob) -> Result<i64, rusqlite::Error> {
        db.execute(
            "INSERT INTO job (
                    user_id,
                    name,
                    job_type,
                    job_status,
                    start_time,
                    end_time,
                    comment)
                VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4,
                    ?5,
                    ?6,
                    ?7);",
            params![
                record.user_id,
                record.name,
                record.job_type,
                record.job_status,
                record.start_time,
                record.end_time,
                record.comment,
            ],
        )?;
        Ok(db.last_insert_rowid())
    }
}

impl RecordUpdate<RecordJob> for TableJob<RecordJob> {
    fn update(db: &Connection, record: &RecordJob) -> Result<usize, rusqlite::Error> {
        db.execute(
            "UPDATE job SET
                    user_id = ?1,
                    name = ?2,
                    job_type = ?3,
                    job_status = ?4,
                    start_time = ?5,
                    end_time = ?6,
                    comment = ?7
                WHERE id = ?8;",
            params![
                record.user_id,
                record.name,
                record.job_type,
                record.job_status,
                record.start_time,
                record.end_time,
                record.comment,
                record.id
            ],
        )
    }
}

impl RecordDelete<RecordJob> for TableJob<RecordJob> {
    fn delete(db: &Connection, record_id: i64) -> Result<usize, rusqlite::Error> {
        TableJobMetadata::delete_by_job(db, record_id)?;
        db.execute("DELETE FROM job WHERE id = ?1;", params![record_id])
    }
}

impl RecordFill<RecordBackupJob> for TableJob<RecordBackupJob> {
    fn fill(row: &rusqlite::Row<'_>, offset: usize) -> Result<RecordBackupJob, rusqlite::Error> {
        Ok(RecordBackupJob {
            id: row.get(offset)?,
            user_id: row.get(offset + 1)?,
            name: row.get(offset + 2)?,
            job_type: row.get(offset + 3)?,
            job_status: row.get(offset + 4)?,
            start_time: row.get(offset + 5)?,
            end_time: row.get(offset + 6)?,
            comment: row.get(offset + 7)?,
            files: vec![],
        })
    }
}

#[cfg(test)]
pub mod tests {
    use crate::backend::database::{
        db::tests::create_test_database, tables::job::table_job::tests::create_table,
    };

    #[test]
    fn suite() {
        let conn = create_test_database();
        create_table(&conn);
    }
}
*/
