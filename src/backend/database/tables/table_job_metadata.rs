use rusqlite::{Connection, Error, params};

use crate::{
    backend::database::tables::table::Table,
    shared::models::database::model_job_metadata::{RecordJobMetadata, RecordJobMetadataJoin},
};

pub struct TableJobMetadata {}

impl Table<RecordJobMetadata, RecordJobMetadataJoin> for TableJobMetadata {
    fn create_table(db: &Connection) -> Result<bool, Error> {
        match db.table_exists(None, "job_metadata") {
            std::result::Result::Ok(exist) => {
                if exist {
                    return Ok(false);
                }
            }
            Err(e) => return Err(e),
        }

        db.execute(
            "CREATE TABLE IF NOT EXISTS job_metadata (
                id INTEGER PRIMARY KEY,
                job_id INTEGER NOT NULL,
                key INTEGER NOT NULL,
                [index] INTEGER NOT NULL,
                value TEXT NOT NULL,
                FOREIGN KEY(job_id) REFERENCES job(id)
            );",
            (),
        )?;
        Ok(true)
    }

    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, Error> {
        Ok(false)
    }

    fn get(db: &Connection, record_id: i64) -> Result<RecordJobMetadata, Error> {
        db.prepare(
            "SELECT
                id,
                job_id,
                key,
                [index],
                value
            FROM job
            WHERE id = ?1",
        )?
        .query_one([record_id], |row| TableJobMetadata::fill(row, 0))
    }

    fn get_join(_db: &Connection, _record_id: i64) -> Result<RecordJobMetadataJoin, Error> {
        todo!()
    }

    fn insert_record(db: &Connection, record: &RecordJobMetadata) -> Result<usize, Error> {
        db.execute(
            "INSERT INTO job (
                    job_id,
                    key,
                    [index],
                    value)
                VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4);",
            params![record.job_id, record.key, record.index, record.value,],
        )
    }

    fn update_record(db: &Connection, record: &RecordJobMetadata) -> Result<usize, Error> {
        db.execute(
            "UPDATE job SET
                    job_id = ?1,
                    key = ?2,
                    [index] = ?3,
                    value = ?4
                WHERE id = ?5;",
            params![
                record.job_id,
                record.key,
                record.index,
                record.value,
                record.id
            ],
        )
    }

    fn delete_record(db: &Connection, record_id: i64) -> Result<usize, Error> {
        db.execute(
            "DELETE FROM job_metadata WHERE id = ?1;",
            params![record_id],
        )
    }

    fn fill(row: &rusqlite::Row<'_>, offset: usize) -> Result<RecordJobMetadata, Error> {
        Ok(RecordJobMetadata {
            id: row.get(offset)?,
            job_id: row.get(offset + 1)?,
            key: row.get(offset + 2)?,
            index: row.get(offset + 3)?,
            value: row.get(offset + 5)?,
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use crate::backend::database::tables::{
        table::Table, table_job::TableJob, table_job_metadata::TableJobMetadata,
        table_user::TableUser,
    };

    fn create() -> rusqlite::Connection {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        // TableJobMetadata depends on TableJob which depends on TableUser
        assert!(
            !conn.table_exists(None, "user").unwrap(),
            "user table should be empty"
        );
        assert!(
            TableUser::create_table(&conn).is_ok(),
            "Failed to create user table"
        );
        assert!(
            conn.table_exists(None, "user").unwrap(),
            "create_table() user reported Ok but table does not exist"
        );

        assert!(
            !conn.table_exists(None, "job").unwrap(),
            "job table should be empty"
        );
        assert!(
            TableJob::create_table(&conn).is_ok(),
            "Failed to create job table"
        );
        assert!(
            conn.table_exists(None, "job").unwrap(),
            "create_table() job reported Ok but table does not exist"
        );

        assert!(
            !conn.table_exists(None, "job_metadata").unwrap(),
            "job_metadata table should be empty"
        );
        assert!(
            TableJobMetadata::create_table(&conn).is_ok(),
            "Failed to create job_metadata table"
        );
        assert!(
            conn.table_exists(None, "job_metadata").unwrap(),
            "create_table() job_metadata reported Ok but table does not exist"
        );
        conn
    }

    #[test]
    fn suite() {
        let _db = create();
    }
}
