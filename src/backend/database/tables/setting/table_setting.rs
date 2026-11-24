use std::marker::PhantomData;

use rusqlite::{Connection, Row, params};

use crate::{
    backend::database::tables::table::{
        RecordFill, RecordInsert, RecordUpdate, TableCreate, TableUpdate,
    },
    shared::models::database::setting::{
        model_setting::{RecordMisc, SettingsKey},
        types_setting::{SettingEmpty, SettingTableVersion},
    },
};

pub struct TableSetting<T> {
    phantom: PhantomData<T>,
}

impl TableCreate<RecordMisc<SettingEmpty>> for TableSetting<RecordMisc<SettingEmpty>> {
    fn create_table(db: &Connection) -> Result<bool, rusqlite::Error> {
        if db.table_exists(None, "setting")? {
            return Ok(false);
        }

        db.execute(
            "CREATE TABLE IF NOT EXISTS setting (
                id INTEGER PRIMARY KEY,
                user_id INTEGER,
                key_primary INTEGER NOT NULL,
                key_secondary TEXT,
                value BLOB NOT NULL,
                FOREIGN KEY(user_id) REFERENCES user(id),
                UNIQUE(user_id, key_primary, key_secondary)
            );",
            (),
        )?;

        Ok(true)
    }
}

impl TableUpdate<RecordMisc<SettingEmpty>> for TableSetting<RecordMisc<SettingEmpty>> {
    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, rusqlite::Error> {
        Ok(false)
    }
}

impl RecordInsert<RecordMisc<i64>> for TableSetting<RecordMisc<i64>> {
    fn insert(db: &Connection, record: &RecordMisc<i64>) -> Result<i64, rusqlite::Error> {
        db.execute(
            "INSERT INTO setting (key_primary, value) VALUES (?1, ?2)",
            params![record.key, record.data],
        )?;
        Ok(db.last_insert_rowid())
    }
}

impl TableSetting<RecordMisc<i64>> {
    pub fn get(db: &Connection) -> Result<RecordMisc<i64>, rusqlite::Error> {
        db.prepare(
            "SELECT
                key_primary,
                value
                FROM setting
                WHERE key_primary = ?1",
        )?
        .query_one([SettingsKey::Version], |row| TableSetting::fill(row, 0))
    }
}

impl RecordUpdate<RecordMisc<i64>> for TableSetting<RecordMisc<i64>> {
    fn update(db: &Connection, record: &RecordMisc<i64>) -> Result<usize, rusqlite::Error> {
        db.execute(
            "UPDATE setting SET value = ?1
                WHERE key_primary = ?2;",
            params![record.data, record.key],
        )
    }
}

impl RecordFill<RecordMisc<i64>> for TableSetting<RecordMisc<i64>> {
    fn fill(row: &Row<'_>, offset: usize) -> Result<RecordMisc<i64>, rusqlite::Error> {
        Ok(RecordMisc::<i64> {
            key: row.get(offset)?,
            data: row.get(offset + 1)?,
        })
    }
}

impl RecordFill<RecordMisc<String>> for TableSetting<RecordMisc<String>> {
    fn fill(row: &Row<'_>, offset: usize) -> Result<RecordMisc<String>, rusqlite::Error> {
        Ok(RecordMisc::<String> {
            key: row.get(offset + 1)?,
            data: row.get(offset + 2)?,
        })
    }
}

impl TableSetting<SettingEmpty> {
    pub fn new_table_init(db: &Connection, version: i64) -> Result<bool, rusqlite::Error> {
        TableSetting::<RecordMisc<SettingTableVersion>>::insert(db, &version.into())?;
        Ok(true)
    }

    pub fn delete_user_setting(
        db: &Connection,
        record_id: i64,
        user_id: i64,
    ) -> Result<usize, rusqlite::Error> {
        db.execute(
            // Only if this user owns this record
            "DELETE FROM setting WHERE id = ?1 AND user_id = ?2;",
            params![record_id, user_id],
        )
    }

    pub const SELECT_BY_USER_ID_AND_KEY_PRIMARY: &str = //
        "SELECT
            id,
            key_secondary,
            value
        FROM setting
        WHERE user_id = ?1 AND key_primary = ?2";

    pub const INSERT_NO_NULLS: &str = //
        "INSERT INTO setting (
            user_id,
            key_primary,
            key_secondary,
            value)
        VALUES (?1, ?2, ?3, ?4)";

    pub const UPDATE_ALL: &str = //
        "UPDATE setting SET
            user_id = ?1,
            key_primary = ?2,
            key_secondary = ?3,
            value = ?4
        WHERE id = ?5";
}

#[cfg(test)]
pub mod tests {
    use crate::{
        backend::database::{
            db::tests::create_test_database,
            tables::{
                setting::table_setting::TableSetting,
                table::{RecordInsert, RecordUpdate, TableCreate},
                tape,
            },
        },
        shared::models::database::setting::{
            model_setting::{RecordMisc, SettingsKey},
            types_setting::{SettingEmpty, SettingTableVersion},
        },
    };

    pub fn create_table(conn: &rusqlite::Connection) {
        tape::table_tape::tests::create_table(conn);
        assert!(
            !conn.table_exists(None, "setting").unwrap(),
            "New table should be empty"
        );
        assert!(
            TableSetting::<RecordMisc<SettingEmpty>>::create_table(conn).is_ok(),
            "Failed to create table"
        );
        assert!(
            conn.table_exists(None, "setting").unwrap(),
            "create_table() reported Ok but table does not exist"
        );
    }

    #[test]
    fn suite() {
        let conn = create_test_database();
        create_table(&conn);

        let mut test_record = RecordMisc::<SettingTableVersion> {
            key: SettingsKey::Version,
            data: 123,
        };
        assert!(
            TableSetting::<RecordMisc<SettingTableVersion>>::insert(&conn, &test_record,).is_ok()
        );

        assert_eq!(
            TableSetting::<RecordMisc<SettingTableVersion>>::get(&conn)
                .unwrap()
                .data,
            test_record.data,
            "Expected data to be set"
        );

        const NEW_DATA: i64 = 555;
        test_record.data = NEW_DATA;
        assert!(
            TableSetting::<RecordMisc<SettingTableVersion>>::update(&conn, &test_record).is_ok(),
            "Failed to update version record"
        );

        assert_eq!(
            TableSetting::<RecordMisc<SettingTableVersion>>::get(&conn)
                .unwrap()
                .data,
            NEW_DATA,
            "Expected data to be updated"
        );
    }
}
