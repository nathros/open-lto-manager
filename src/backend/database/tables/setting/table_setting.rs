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
        match db.table_exists(None, "setting") {
            std::result::Result::Ok(exist) => {
                if exist {
                    return Ok(false);
                }
            }
            Err(e) => return Err(e),
        }

        db.execute(
            "CREATE TABLE IF NOT EXISTS setting (
                id INTEGER PRIMARY KEY,
                key INTEGER NOT NULL,
                value BLOB NOT NULL
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
            "INSERT INTO setting (key, value) VALUES (?1, ?2)",
            params![record.key, record.data],
        )?;
        Ok(db.last_insert_rowid())
    }
}

impl TableSetting<RecordMisc<i64>> {
    pub fn get(db: &Connection) -> Result<RecordMisc<i64>, rusqlite::Error> {
        db.prepare(
            "SELECT
                key,
                value
                FROM setting
                WHERE key = ?1",
        )?
        .query_one([SettingsKey::Version], |row| TableSetting::fill(row, 0))
    }
}

impl RecordUpdate<RecordMisc<i64>> for TableSetting<RecordMisc<i64>> {
    fn update(db: &Connection, record: &RecordMisc<i64>) -> Result<usize, rusqlite::Error> {
        db.execute(
            "UPDATE setting SET value = ?1
                WHERE key = ?2;",
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
}

/*
impl Table<RecordVersion, RecordVersion> for TableVersion {
    fn create_table(db: &Connection) -> Result<bool, Error> {

    }

    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, Error> {

    }

    fn get(db: &Connection, _record_id: i64) -> Result<RecordVersion, Error> {
        db.prepare("SELECT * FROM version WHERE id = ?1")?
            .query_one([1], |row| TableVersion::fill(row, 0))
    }

    fn insert_record(db: &Connection, record: &RecordVersion) -> Result<i64, Error> {
        db.execute(
            "INSERT INTO version (version_number) VALUES (?1)",
            params![record.version_number],
        )?;
        Ok(db.last_insert_rowid())
    }

    fn insert_batch(db: &Connection, records: &[RecordVersion]) -> Result<usize, Error> {
        let mut count = 0;
        let mut prepared = db.prepare("INSERT INTO version (version_number) VALUES (?1)")?;
        for record in records {
            count += prepared.execute(params![record.version_number])?;
        }
        Ok(count)
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

    fn clear_table(_db: &Connection) -> Result<usize, rusqlite::Error> {
        Err(Error::InvalidParameterName(
            "Do not clear Version table".to_string(),
        ))
    }

    fn fill(row: &Row<'_>, _offset: usize) -> Result<RecordVersion, Error> {
        Ok(RecordVersion {
            version_number: row.get(1)?,
        })
    }
}
*/
#[cfg(test)]
mod tests {
    use crate::{
        backend::database::{
            db::tests::create_test_database,
            tables::{
                setting::table_setting::TableSetting,
                table::{RecordInsert, RecordUpdate, TableCreate},
            },
        },
        shared::models::database::setting::{
            model_setting::{RecordMisc, SettingsKey},
            types_setting::{SettingEmpty, SettingTableVersion},
        },
    };

    fn create_table(conn: &rusqlite::Connection) {
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
