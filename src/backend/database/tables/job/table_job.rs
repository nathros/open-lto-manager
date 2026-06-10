use std::marker::PhantomData;

use rusqlite::{Connection, params};

use crate::{
    backend::database::tables::{
        job_metadata::table_job_metadata::TableJobMetadata,
        table::{
            RecordDelete, RecordFill, RecordInsert, RecordRead, RecordUpdate, TableCreate,
            TableUpdate,
        },
    },
    shared::models::database::job::model_job::RecordJob,
};

pub struct TableJob<T = RecordJob> {
    phantom: PhantomData<T>,
}

impl TableCreate<RecordJob> for TableJob<RecordJob> {
    fn create_table(db: &Connection) -> Result<bool, rusqlite::Error> {
        match db.table_exists(None, "job") {
            std::result::Result::Ok(exist) => {
                if exist {
                    return Ok(false);
                }
            }
            Err(e) => return Err(e),
        }

        db.execute(
            "CREATE TABLE IF NOT EXISTS job (
                id INTEGER PRIMARY KEY,
                user_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                job_type INTEGER NOT NULL,
                job_status INTEGER NOT NULL,
                start_time BIGINT NOT NULL,
                end_time BIGINT NOT NULL,
                comment TEXT NOT NULL,
                FOREIGN KEY(user_id) REFERENCES user(id)
            );",
            (),
        )?;
        Ok(true)
    }
}

impl TableUpdate<RecordJob> for TableJob<RecordJob> {
    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, rusqlite::Error> {
        Ok(false)
    }
}

impl RecordRead<RecordJob> for TableJob<RecordJob> {
    fn get(db: &Connection, record_id: i64) -> Result<RecordJob, rusqlite::Error> {
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

impl RecordFill<RecordJob> for TableJob<RecordJob> {
    fn fill(row: &rusqlite::Row<'_>, offset: usize) -> Result<RecordJob, rusqlite::Error> {
        Ok(RecordJob {
            id: row.get(offset)?,
            user_id: row.get(offset + 1)?,
            name: row.get(offset + 2)?,
            job_type: row.get(offset + 3)?,
            job_status: row.get(offset + 4)?,
            start_time: row.get(offset + 5)?,
            end_time: row.get(offset + 6)?,
            comment: row.get(offset + 7)?,
        })
    }
}

impl TableJob<RecordJob> {
    pub fn get_all(db: &Connection) -> Result<Vec<RecordJob>, rusqlite::Error> {
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
                ORDER BY id",
        )?
        .query_map([], |row| TableJob::fill(row, 0))?
        .collect::<Result<Vec<RecordJob>, rusqlite::Error>>()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        backend::database::tables::{
            job::table_job::TableJob, table::TableCreate, user::table_user,
        },
        shared::models::database::job::model_job::RecordJob,
    };

    pub fn create_table(conn: &rusqlite::Connection) {
        // TableJob depends on TableUser
        table_user::tests::create_table(conn);

        assert!(
            !conn.table_exists(None, "job").unwrap(),
            "New table should be empty"
        );
        assert!(
            TableJob::<RecordJob>::create_table(conn).is_ok(),
            "Failed to create job table"
        );
        assert!(
            conn.table_exists(None, "job").unwrap(),
            "create_table() job reported Ok but table does not exist"
        );
    }

    #[test]
    fn suite() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        create_table(&conn);
    }
}
