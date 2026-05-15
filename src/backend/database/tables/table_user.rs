use chrono::Local;
use rusqlite::{Connection, Error, params};

use crate::{
    backend::database::tables::table::Table,
    shared::models::database::model_user::{ColourMode, RecordUser, RecordUserWithRoles},
};

pub struct TableUser {}

impl Table<RecordUser, RecordUserWithRoles> for TableUser {
    fn create_table(db: &Connection) -> Result<bool, Error> {
        match db.table_exists(None, "user") {
            std::result::Result::Ok(exist) => {
                if exist {
                    return Ok(false);
                }
            }
            Err(e) => return Err(e),
        }

        db.execute(
            "CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                description TEXT,
                hash TEXT NOT NULL,
                salt TEXT NOT NULL,
                enabled BOOLEAN NOT NULL,
                created BIGINT NOT NULL,
                language INTEGER NOT NULL,
                avatar TEXT,
                system_theme INTEGER NOT NULL,
                icon_theme TEXT NOT NULL,
                fm_theme TEXT NOT NULL,
                accent_colour TEXT NOT NULL
            );",
            (),
        )?;

        Self::insert_record(
            db,
            &RecordUser {
                id: 0,
                username: "admin".to_string(),
                description: "Admin".to_string(),
                hash: "".to_string(),
                salt: "".to_string(),
                enabled: true,
                created: Local::now(),
                language: 0,
                avatar: "".to_string(),
                system_theme: ColourMode::System,
                icon_theme: "".to_string(),
                fm_theme: "".to_string(),
                accent_colour: "".to_string(),
            },
        )?;

        Ok(true)
    }

    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, Error> {
        Ok(false)
    }

    fn get(db: &Connection, record_id: i64) -> Result<RecordUser, Error> {
        db.prepare(
            "SELECT
                id,
                username,
                description,
                hash,
                salt,
                enabled,
                created,
                language,
                avatar,
                system_theme,
                icon_theme,
                fm_theme,
                accent_colour
            FROM user
            WHERE id = ?1",
        )?
        .query_one([record_id], |row| TableUser::fill(row, 0))
    }

    fn get_join(_db: &Connection, _record_id: i64) -> Result<RecordUserWithRoles, Error> {
        todo!()
    }

    fn insert_record(db: &Connection, record: &RecordUser) -> Result<i64, Error> {
        db.execute(
            "INSERT INTO user (
                    username,
                    description,
                    hash,
                    salt,
                    enabled,
                    created,
                    language,
                    avatar,
                    system_theme,
                    icon_theme,
                    fm_theme,
                    accent_colour)
                VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4,
                    ?5,
                    ?6,
                    ?7,
                    ?8,
                    ?9,
                    ?10,
                    ?11,
                    ?12);",
            params![
                record.username,
                record.description,
                record.hash,
                record.salt,
                record.enabled,
                record.created,
                record.language,
                record.avatar,
                record.system_theme,
                record.icon_theme,
                record.fm_theme,
                record.accent_colour,
            ],
        )?;
        Ok(db.last_insert_rowid())
    }

    fn insert_batch(db: &Connection, records: &[RecordUser]) -> Result<usize, Error> {
        let mut count = 0;
        let mut prepared = db.prepare(
            "INSERT INTO user (
                    username,
                    description,
                    hash,
                    salt,
                    enabled,
                    created,
                    language,
                    avatar,
                    system_theme,
                    icon_theme,
                    fm_theme,
                    accent_colour)
                VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4,
                    ?5,
                    ?6,
                    ?7,
                    ?8,
                    ?9,
                    ?10,
                    ?11,
                    ?12);",
        )?;
        for record in records {
            count += prepared.execute(params![
                record.username,
                record.description,
                record.hash,
                record.salt,
                record.enabled,
                record.created,
                record.language,
                record.avatar,
                record.system_theme,
                record.icon_theme,
                record.fm_theme,
                record.accent_colour,
            ])?;
        }
        Ok(count)
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
                    system_theme = ?9,
                    icon_theme = ?10,
                    fm_theme = ?11,
                    accent_colour = ?12
                WHERE id = ?13;",
            params![
                record.username,
                record.description,
                record.hash,
                record.salt,
                record.enabled,
                record.created,
                record.language,
                record.avatar,
                record.system_theme,
                record.icon_theme,
                record.fm_theme,
                record.accent_colour,
                record.id
            ],
        )
    }

    fn delete_record(db: &Connection, record_id: i64) -> Result<usize, Error> {
        db.execute("DELETE FROM user WHERE id = ?1;", params![record_id])
    }

    fn clear_table(db: &Connection) -> Result<usize, rusqlite::Error> {
        db.execute("DELETE FROM user;", ())
    }

    fn fill(row: &rusqlite::Row<'_>, offset: usize) -> Result<RecordUser, Error> {
        Ok(RecordUser {
            id: row.get(offset)?,
            username: row.get(offset + 1)?,
            description: row.get(offset + 2)?,
            hash: row.get(offset + 3)?,
            salt: row.get(offset + 4)?,
            enabled: row.get(offset + 5)?,
            created: row.get(offset + 6)?,
            language: row.get(offset + 7)?,
            avatar: row.get(offset + 8)?,
            system_theme: row.get(offset + 9)?,
            icon_theme: row.get(offset + 10)?,
            fm_theme: row.get(offset + 11)?,
            accent_colour: row.get(offset + 12)?,
        })
    }
}

impl TableUser {
    pub fn get_all(db: &Connection) -> Result<Vec<RecordUser>, rusqlite::Error> {
        db.prepare(
            "SELECT
                    id,
                    username,
                    description,
                    hash,
                    salt,
                    enabled,
                    created,
                    language,
                    avatar,
                    system_theme,
                    icon_theme,
                    fm_theme,
                    accent_colour
                FROM user
                ORDER BY id",
        )?
        .query_map([], |row| TableUser::fill(row, 0))?
        .collect::<Result<Vec<RecordUser>, rusqlite::Error>>()
    }
}

#[cfg(test)]
pub mod tests {
    #![allow(clippy::unwrap_used)]
    use chrono::Local;

    use crate::{
        backend::database::tables::{table::Table, table_user::TableUser},
        shared::models::database::model_user::{ColourMode, RecordUser},
    };

    pub fn create(conn: &rusqlite::Connection) {
        assert!(
            !conn.table_exists(None, "user").unwrap(),
            "New table should be empty"
        );
        assert!(
            TableUser::create_table(conn).is_ok(),
            "Failed to create table"
        );
        assert!(
            conn.table_exists(None, "user").unwrap(),
            "create_table() reported Ok but table does not exist"
        );
    }

    fn insert(db: &rusqlite::Connection) {
        let mut new_user = RecordUser {
            id: 0,
            username: "test1".to_string(),
            description: "test2".to_string(),
            hash: "test3".to_string(),
            salt: "test4".to_string(),
            enabled: true,
            created: Local::now(),
            language: 0,
            avatar: "test5".to_string(),
            system_theme: ColourMode::Dark,
            icon_theme: "test".to_string(),
            fm_theme: "test".to_string(),
            accent_colour: "test".to_string(),
        };

        let insert_result = TableUser::insert_record(db, &new_user);
        if insert_result.is_err() {
            println!("--insert_result: {:?}", insert_result);
        }
        assert!(insert_result.is_ok(), "Failed to insert new user");
        let get_inserted_user_result = TableUser::get(db, 2); // 1 is Admin
        if insert_result.is_err() {
            println!("--get_inserted_user_result: {:?}", get_inserted_user_result);
        }
        assert!(
            get_inserted_user_result.is_ok(),
            "Failed to get newly inserted new user"
        );
        let inserted_user = get_inserted_user_result.unwrap();
        new_user.id = inserted_user.id; // New record will not have assigned id
        assert_eq!(inserted_user, new_user);
    }

    #[test]
    fn suite() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        create(&conn);
        insert(&conn);
    }
}
