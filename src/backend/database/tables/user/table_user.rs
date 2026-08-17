use std::marker::PhantomData;

use rusqlite::{Connection, Error, params};

use crate::{
    backend::database::tables::table::{
        RecordDelete, RecordFill, RecordInsert, RecordRead, RecordUpdate, TableCreate, TableUpdate,
    },
    shared::models::database::user::model_user_sensitive::RecordUser,
};

pub struct TableUser<T = RecordUser> {
    phantom: PhantomData<T>,
}

impl TableCreate<RecordUser> for TableUser<RecordUser> {
    fn create_table(db: &Connection) -> Result<bool, rusqlite::Error> {
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
                algorithm INTEGER NOT NULL,
                enabled BOOLEAN NOT NULL,
                created BIGINT NOT NULL,
                language INTEGER NOT NULL,
                avatar TEXT,
                system_theme INTEGER NOT NULL,
                icon_theme INTEGER NOT NULL,
                file_theme INTEGER NOT NULL,
                accent_colour TEXT NOT NULL
            );",
            (),
        )?;

        Self::insert(
            db,
            &RecordUser::create("admin".to_string(), "Admin".to_string(), "admin"),
        )?;

        Ok(true)
    }
}

impl TableUpdate<RecordUser> for TableUser<RecordUser> {
    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, rusqlite::Error> {
        Ok(false)
    }
}

impl RecordRead<RecordUser> for TableUser<RecordUser> {
    fn get(db: &Connection, record_id: i64) -> Result<RecordUser, rusqlite::Error> {
        db.prepare(
            "SELECT
                id,
                username,
                description,
                hash,
                salt,
                algorithm,
                enabled,
                created,
                language,
                avatar,
                system_theme,
                icon_theme,
                file_theme,
                accent_colour
            FROM user
            WHERE id = ?1",
        )?
        .query_one([record_id], |row| TableUser::fill(row, 0))
    }
}

impl RecordInsert<RecordUser> for TableUser<RecordUser> {
    fn insert(db: &Connection, record: &RecordUser) -> Result<i64, rusqlite::Error> {
        db.execute(
            "INSERT INTO user (
                    username,
                    description,
                    hash,
                    salt,
                    algorithm,
                    enabled,
                    created,
                    language,
                    avatar,
                    system_theme,
                    icon_theme,
                    file_theme,
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
                    ?12,
                    ?13);",
            params![
                record.username,
                record.description,
                record.hash,
                record.salt,
                record.algorithm,
                record.enabled,
                record.created,
                record.language,
                record.avatar,
                record.system_theme,
                record.icon_theme,
                record.file_theme,
                record.accent_colour,
            ],
        )?;
        Ok(db.last_insert_rowid())
    }
}

impl RecordUpdate<RecordUser> for TableUser<RecordUser> {
    fn update(db: &Connection, record: &RecordUser) -> Result<usize, rusqlite::Error> {
        db.execute(
            "UPDATE user SET
                    username = ?1,
                    description = ?2,
                    hash = ?3,
                    salt = ?4,
                    algorithm = ?5,
                    enabled = ?6,
                    created = ?7,
                    language = ?8,
                    avatar = ?9,
                    system_theme = ?10,
                    icon_theme = ?11,
                    file_theme = ?12,
                    accent_colour = ?13
                WHERE id = ?14;",
            params![
                record.username,
                record.description,
                record.hash,
                record.salt,
                record.algorithm,
                record.enabled,
                record.created,
                record.language,
                record.avatar,
                record.system_theme,
                record.icon_theme,
                record.file_theme,
                record.accent_colour,
                record.id
            ],
        )
    }
}

impl RecordDelete<RecordUser> for TableUser<RecordUser> {
    fn delete(db: &Connection, record_id: i64) -> Result<usize, rusqlite::Error> {
        db.execute("DELETE FROM user WHERE id = ?1;", params![record_id])
    }
}

impl RecordFill<RecordUser> for TableUser<RecordUser> {
    fn fill(row: &rusqlite::Row<'_>, offset: usize) -> Result<RecordUser, rusqlite::Error> {
        Ok(RecordUser {
            id: row.get(offset)?,
            username: row.get(offset + 1)?,
            description: row.get(offset + 2)?,
            hash: row.get(offset + 3)?,
            salt: row.get(offset + 4)?,
            algorithm: row.get(offset + 5)?,
            enabled: row.get(offset + 6)?,
            created: row.get(offset + 7)?,
            language: row.get(offset + 8)?,
            avatar: row.get(offset + 9)?,
            system_theme: row.get(offset + 10)?,
            icon_theme: row.get(offset + 11)?,
            file_theme: row.get(offset + 12)?,
            accent_colour: row.get(offset + 13)?,
        })
    }
}

impl TableUser<RecordUser> {
    pub fn get_by_username(db: &Connection, username: String) -> Result<RecordUser, Error> {
        db.prepare(
            "SELECT
                id,
                username,
                description,
                hash,
                salt,
                algorithm,
                enabled,
                created,
                language,
                avatar,
                system_theme,
                icon_theme,
                file_theme,
                accent_colour
            FROM user
            WHERE username = ?1",
        )?
        .query_one([username], |row| TableUser::fill(row, 0))
    }

    pub fn get_all(db: &Connection) -> Result<Vec<RecordUser>, rusqlite::Error> {
        db.prepare(
            "SELECT
                    id,
                    username,
                    description,
                    hash,
                    salt,
                    algorithm,
                    enabled,
                    created,
                    language,
                    avatar,
                    system_theme,
                    icon_theme,
                    file_theme,
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
    use chrono::Local;

    use crate::{
        backend::database::{
            db::tests::create_test_database,
            tables::{
                table::{RecordInsert, RecordRead, TableCreate},
                user::table_user::TableUser,
            },
        },
        shared::models::database::user::{
            model_user::{ColourMode, FileTheme, IconTheme},
            model_user_sensitive::{HashAlgorithm, RecordUser},
        },
    };

    const TEST_USERNAME: &str = "test1";

    pub fn create_table(conn: &rusqlite::Connection) {
        assert!(
            !conn.table_exists(None, "user").unwrap(),
            "New table should be empty"
        );
        assert!(
            TableUser::<RecordUser>::create_table(conn).is_ok(),
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
            username: TEST_USERNAME.to_string(),
            description: "test2".to_string(),
            hash: "test3".to_string(),
            salt: "test4".to_string(),
            algorithm: HashAlgorithm::latest(),
            enabled: true,
            created: Local::now(),
            language: 0,
            avatar: "test5".to_string(),
            system_theme: ColourMode::Dark,
            icon_theme: IconTheme::Tabler,
            file_theme: FileTheme::Breeze,
            accent_colour: "test".to_string(),
        };

        let insert_result = TableUser::<RecordUser>::insert(db, &new_user);
        if insert_result.is_err() {
            println!("--insert_result: {:?}", insert_result);
        }
        assert!(insert_result.is_ok(), "Failed to insert new user");
        let get_inserted_user_result = TableUser::<RecordUser>::get(db, 2); // 1 is Admin
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

    fn get(db: &rusqlite::Connection) {
        assert!(TableUser::get_by_username(db, "xzy".to_string()).is_err()); // Does not exist
        let existing_user = TableUser::get_by_username(db, TEST_USERNAME.to_string());
        assert_eq!(
            existing_user.unwrap().username,
            TEST_USERNAME,
            "Expected username to match"
        );
    }

    #[test]
    fn suite() {
        let conn = create_test_database();
        create_table(&conn);
        insert(&conn);
        get(&conn);
    }
}
