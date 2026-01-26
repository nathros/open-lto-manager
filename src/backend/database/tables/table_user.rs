use rusqlite::{params, Connection, Error};

use crate::{
    backend::database::tables::table::Table,
    shared::models::database::model_user::{RecordUser, RecordUserWithRoles},
};

pub struct TableUser {}

impl Table<RecordUser, RecordUserWithRoles> for TableUser {
    fn create_table(db: &Connection) -> Result<bool, Error> {
        match db.table_exists(None, "user") {
            std::result::Result::Ok(exist) => {
                if exist == true {
                    return Ok(false);
                }
            }
            Err(e) => return Err(e),
        }

        if let Err(e) = db.execute(
            "CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                description TEXT,
                hash TEXT NOT NULL,
                salt TEXT NOT NULL,
                enabled BOOLEAN NOT NULL,
                created BIGINT NOT NULL,
                language INTEGER NOT NULL,
                avatar TEXT
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

    fn get(_db: &Connection, _record_id: i64) -> Result<RecordUser, Error> {
        todo!()
    }

    fn get_join(_db: &Connection, _record_id: i64) -> Result<RecordUserWithRoles, Error> {
        todo!()
    }

    fn insert_record(db: &Connection, record: &RecordUser) -> Result<usize, Error> {
        db.execute(
            "INSERT INTO user (
                    username,
                    description,
                    hash,
                    salt,
                    enabled,
                    created,
                    language,
                    avatar)
                VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4,
                    ?5,
                    ?6,
                    ?7,
                    ?8);",
            params![
                record.username,
                record.description,
                record.hash,
                record.salt,
                record.enabled,
                record.created,
                record.language,
                record.avatar,
            ],
        )
    }

    fn update_record(db: &Connection, record: &RecordUser) -> Result<usize, Error> {
        db.execute(
            "UPDATE user SET
                    username = ?1,
                    description = ?2,
                    hash = ?3,
                    salt = ?4,
                    enabled = ?5,
                    created = ?6,
                    language = ?7,
                    avatar = ?8,
                WHERE id = ?9;",
            params![
                record.username,
                record.description,
                record.hash,
                record.salt,
                record.enabled,
                record.created,
                record.language,
                record.avatar,
                record.id
            ],
        )
    }

    fn delete_record(db: &Connection, record_id: i64) -> Result<usize, Error> {
        db.execute("DELETE FROM user WHERE id = ?1;", params![record_id])
    }

    fn fill(row: &rusqlite::Row<'_>, offset: usize) -> Result<RecordUser, Error> {
        Ok(RecordUser {
            id: row.get(offset + 0)?,
            username: row.get(offset + 1)?,
            description: row.get(offset + 2)?,
            hash: row.get(offset + 3)?,
            salt: row.get(offset + 4)?,
            enabled: row.get(offset + 5)?,
            created: row.get(offset + 6)?,
            language: row.get(offset + 7)?,
            avatar: row.get(offset + 8)?,
        })
    }
}
