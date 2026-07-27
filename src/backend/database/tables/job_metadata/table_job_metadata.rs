use std::marker::PhantomData;

use rusqlite::{Connection, Error, params};

use crate::{
    backend::database::tables::table::{
        RecordDelete, RecordFill, RecordInsert, RecordInsertBatch, RecordRead, RecordUpdate,
        TableCreate, TableUpdate,
    },
    shared::models::database::job_metadata::model_job_metadata::RecordJobMetadata,
};

pub struct TableJobMetadata<T = RecordJobMetadata> {
    phantom: PhantomData<T>,
}

impl TableCreate<RecordJobMetadata> for TableJobMetadata<RecordJobMetadata> {
    fn create_table(db: &Connection) -> Result<bool, rusqlite::Error> {
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
}

impl TableUpdate<RecordJobMetadata> for TableJobMetadata<RecordJobMetadata> {
    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, rusqlite::Error> {
        Ok(false)
    }
}

impl RecordRead<RecordJobMetadata> for TableJobMetadata<RecordJobMetadata> {
    fn get(db: &Connection, record_id: i64) -> Result<RecordJobMetadata, rusqlite::Error> {
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
}

impl RecordInsert<RecordJobMetadata> for TableJobMetadata<RecordJobMetadata> {
    fn insert(db: &Connection, record: &RecordJobMetadata) -> Result<i64, rusqlite::Error> {
        db.execute(
            "INSERT INTO job_metadata (
                    job_id,
                    key,
                    [index],
                    value)
                VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4);",
            params![record.job_id, record.key, record.index, record.value],
        )?;
        Ok(db.last_insert_rowid())
    }
}

impl RecordInsertBatch<RecordJobMetadata> for TableJobMetadata<RecordJobMetadata> {
    fn insert_batch(
        db: &Connection,
        records: &[RecordJobMetadata],
    ) -> Result<usize, rusqlite::Error> {
        let mut count = 0;
        let mut prepared = db.prepare(
            "INSERT INTO job_metadata (
                    job_id,
                    key,
                    [index],
                    value)
                VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4);",
        )?;
        for record in records {
            count += prepared.execute(params![
                record.job_id,
                record.key,
                record.index,
                record.value
            ])?;
        }
        Ok(count)
    }
}

impl RecordUpdate<RecordJobMetadata> for TableJobMetadata<RecordJobMetadata> {
    fn update(db: &Connection, record: &RecordJobMetadata) -> Result<usize, rusqlite::Error> {
        db.execute(
            "UPDATE job_metadata SET
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
}

impl RecordDelete<RecordJobMetadata> for TableJobMetadata<RecordJobMetadata> {
    fn delete(db: &Connection, record_id: i64) -> Result<usize, rusqlite::Error> {
        db.execute(
            "DELETE FROM job_metadata WHERE id = ?1;",
            params![record_id],
        )
    }
}

impl RecordFill<RecordJobMetadata> for TableJobMetadata<RecordJobMetadata> {
    fn fill(row: &rusqlite::Row<'_>, offset: usize) -> Result<RecordJobMetadata, rusqlite::Error> {
        Ok(RecordJobMetadata {
            id: row.get(offset)?,
            job_id: row.get(offset + 1)?,
            key: row.get(offset + 2)?,
            index: row.get(offset + 3)?,
            value: row.get(offset + 4)?,
        })
    }
}

impl TableJobMetadata<RecordJobMetadata> {
    pub fn get_all(db: &Connection) -> Result<Vec<RecordJobMetadata>, rusqlite::Error> {
        db.prepare(
            "SELECT
                    id,
                    job_id,
                    key,
                    [index],
                    value
                FROM job_metadata
                ORDER BY id",
        )?
        .query_map([], |row| TableJobMetadata::fill(row, 0))?
        .collect::<Result<Vec<RecordJobMetadata>, rusqlite::Error>>()
    }

    pub fn delete_by_job(db: &Connection, job_id: i64) -> Result<usize, Error> {
        db.execute(
            "DELETE FROM job_metadata WHERE job_id = ?1;",
            params![job_id],
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        backend::database::{
            db::tests::create_test_database,
            tables::{
                job::table_job, job_metadata::table_job_metadata::TableJobMetadata,
                table::TableCreate,
            },
        },
        shared::models::database::job_metadata::model_job_metadata::RecordJobMetadata,
    };

    fn create_table(conn: &rusqlite::Connection) {
        // TableJobMetadata depends on TableJob which depends on TableUser
        table_job::tests::create_table(conn);

        assert!(
            !conn.table_exists(None, "job_metadata").unwrap(),
            "job_metadata table should be empty"
        );
        assert!(
            TableJobMetadata::<RecordJobMetadata>::create_table(conn).is_ok(),
            "Failed to create job_metadata table"
        );
        assert!(
            conn.table_exists(None, "job_metadata").unwrap(),
            "create_table() job_metadata reported Ok but table does not exist"
        );
    }

    #[test]
    fn suite() {
        let conn = create_test_database();
        create_table(&conn);
    }
}
