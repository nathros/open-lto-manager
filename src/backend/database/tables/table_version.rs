use rusqlite::{params, Connection, Error, Row};

use crate::shared::models::database::model_version::RecordVersion;

use super::table::Table;

pub struct TableVersion {}

impl Table<RecordVersion, RecordVersion> for TableVersion {
    fn create_table(db: &Connection) -> Result<bool, Error> {
        match db.table_exists(None, "version") {
            std::result::Result::Ok(exist) => {
                if exist == true {
                    return Ok(false);
                }
            }
            Err(e) => return Err(e),
        }

        if let Err(e) = db.execute(
            "CREATE TABLE IF NOT EXISTS version (
                id INTEGER PRIMARY KEY,
                version_number INTEGER NOT NULL
            );",
            (),
        ) {
            return Err(e); // Failed to create table
        }

        return Ok(true);
    }

    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, Error> {
        Ok(false)
    }

    fn get(db: &Connection, _record_id: i64) -> Result<RecordVersion, Error> {
        db.prepare("SELECT * FROM version WHERE id = ?1")?
            .query_one([1], |row| TableVersion::fill(row, 0))
    }

    fn get_join(db: &Connection, record_id: i64) -> Result<RecordVersion, Error> {
        TableVersion::get(db, record_id)
    }

    fn insert_record(db: &Connection, record: &RecordVersion) -> Result<usize, Error> {
        db.execute(
            "INSERT INTO version (version_number) VALUES (?1)",
            params![record.version_number],
        )
    }

    fn update_record(db: &Connection, _record: &RecordVersion) -> Result<usize, Error> {
        db.execute(
            "UPDATE version SET version = ?1
                WHERE id = 1;",
            params![1],
        )
    }

    fn delete_record(_db: &Connection, _record_id: i64) -> Result<usize, Error> {
        Err(Error::InvalidParameterName(
            "Do not remove from Version table".to_string(),
        ))
    }

    fn fill(row: &Row<'_>, _offset: usize) -> Result<RecordVersion, Error> {
        Ok(RecordVersion {
            version_number: row.get(1)?,
        })
    }
}

#[cfg(test)]
mod tests {}
