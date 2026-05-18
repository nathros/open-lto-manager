use rusqlite::{Connection, Error, Row, params};

use crate::shared::models::database::model_label_preset::{
    RecordLabelPreset, RecordLabelPresetJoin,
};

use super::table::Table;

pub struct TableLabelPreset {}

impl Table<RecordLabelPreset, RecordLabelPresetJoin> for TableLabelPreset {
    fn create_table(db: &Connection) -> Result<bool, Error> {
        match db.table_exists(None, "label_preset") {
            std::result::Result::Ok(exist) => {
                if exist {
                    return Ok(false);
                }
            }
            Err(e) => return Err(e),
        }

        db.execute(
            "CREATE TABLE IF NOT EXISTS label_preset (
                id INTEGER PRIMARY KEY,
                user_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                settings TEXT NOT NULL,
                FOREIGN KEY(user_id) REFERENCES user(id)
            );",
            (),
        )?;

        Ok(true)
    }

    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, Error> {
        Ok(false)
    }

    fn get(db: &Connection, record_id: i64) -> Result<RecordLabelPreset, Error> {
        db.prepare(
            "SELECT
                    id,
                    user_id,
                    name,
                    settings
                FROM label_preset
                    WHERE id = ?1",
        )?
        .query_one([record_id], |row| TableLabelPreset::fill(row, 0))
    }

    fn get_join(_db: &Connection, _record_id: i64) -> Result<RecordLabelPresetJoin, Error> {
        todo!()
    }

    fn insert_record(db: &Connection, record: &RecordLabelPreset) -> Result<i64, Error> {
        db.execute(
            "INSERT INTO label_preset (
                    user_id,
                    name,
                    settings)
                VALUES (?1, ?2, ?3)",
            params![record.user_id, record.name, record.options],
        )?;
        Ok(db.last_insert_rowid())
    }

    fn insert_batch(db: &Connection, records: &[RecordLabelPreset]) -> Result<usize, Error> {
        let mut count = 0;
        let mut prepared = db.prepare(
            "INSERT INTO label_preset (
                    user_id,
                    name,
                    settings)
                VALUES (?1, ?2, ?3)",
        )?;
        for record in records {
            count += prepared.execute(params![record.user_id, record.name, record.options])?;
        }
        Ok(count)
    }

    fn update_record(db: &Connection, record: &RecordLabelPreset) -> Result<usize, Error> {
        db.execute(
            "UPDATE label_preset SET
                    user_id = ?1,
                    name = ?2,
                    settings = ?3
                WHERE id = ?4;",
            params![record.user_id, record.name, record.options, record.id],
        )
    }

    fn delete_record(db: &Connection, record_id: i64) -> Result<usize, Error> {
        db.execute(
            "DELETE FROM label_preset WHERE id = ?1;",
            params![record_id],
        )
    }

    fn clear_table(db: &Connection) -> Result<usize, rusqlite::Error> {
        db.execute("DELETE FROM label_preset;", ())
    }

    fn fill(row: &Row<'_>, _offset: usize) -> Result<RecordLabelPreset, Error> {
        Ok(RecordLabelPreset {
            id: row.get(0)?,
            user_id: row.get(1)?,
            name: row.get(2)?,
            options: row.get(3)?,
        })
    }
}

impl TableLabelPreset {
    pub fn get_user_presets(
        db: &Connection,
        user_id: i64,
    ) -> Result<Vec<RecordLabelPreset>, rusqlite::Error> {
        db.prepare(
            "SELECT
                    id,
                    user_id,
                    name,
                    settings
                FROM label_preset
                    WHERE user_id = ?1",
        )?
        .query_map([user_id], |row| TableLabelPreset::fill(row, 0))?
        .collect::<Result<Vec<RecordLabelPreset>, rusqlite::Error>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        backend::database::tables::{
            table::Table,
            table_label_preset::TableLabelPreset,
            table_user::{self, TableUser},
        },
        shared::models::database::model_label_preset::{LabelOptions, RecordLabelPreset},
    };

    fn create(conn: &rusqlite::Connection) {
        // TableLabelPreset depends on TableUser, so this must be created first
        table_user::tests::create_table(conn);

        assert!(
            !conn.table_exists(None, "label_preset").unwrap(),
            "New table should be empty"
        );
        assert!(
            TableLabelPreset::create_table(conn).is_ok(),
            "Failed to create table"
        );
        assert!(
            conn.table_exists(None, "label_preset").unwrap(),
            "create_table() reported Ok but table does not exist"
        );
    }

    #[test]
    fn create_table() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        create(&conn);
    }

    #[test]
    fn insert_and_update() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        create(&conn);
        let users = TableUser::get_all(&conn).unwrap();
        let user = users.first().unwrap(); // User used for this test

        assert!(
            TableLabelPreset::get_user_presets(&conn, user.id)
                .unwrap()
                .is_empty(),
            "Expected user to have no presets"
        );
        let mut preset: RecordLabelPreset = RecordLabelPreset {
            id: 0, // Must be updated later for check if insert was done correctly
            user_id: user.id,
            name: "test1".to_string(),
            options: LabelOptions::default(),
        };
        assert!(TableLabelPreset::insert_record(&conn, &preset).is_ok());
        let user_presets = TableLabelPreset::get_user_presets(&conn, user.id).unwrap();
        let user_preset = user_presets.first().unwrap();
        preset.id = user_preset.id; // Before insert id cannot be known
        assert_eq!(preset, *user_preset, "Expected new preset to be the same");

        // Update
        preset.name = "new_name".to_string();
        preset.options.stroke_inner = 99_f64;
        assert!(TableLabelPreset::update_record(&conn, &preset).is_ok());
        let user_presets = TableLabelPreset::get_user_presets(&conn, user.id).unwrap();
        let user_preset = user_presets.first().unwrap();
        assert_eq!(preset, *user_preset, "Expected new preset to be the same");
    }

    #[test]
    fn delete() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        create(&conn);
        let users = TableUser::get_all(&conn).unwrap();
        let user = users.first().unwrap(); // User used for this test
        assert!(
            TableLabelPreset::get_user_presets(&conn, user.id)
                .unwrap()
                .is_empty(),
            "Expected user to have no presets"
        );
        let preset: RecordLabelPreset = RecordLabelPreset {
            id: 0, // Must be updated later for check if insert was done correctly
            user_id: user.id,
            name: "test1".to_string(),
            options: LabelOptions::default(),
        };
        let new_id = TableLabelPreset::insert_record(&conn, &preset).unwrap();
        assert!(
            TableLabelPreset::get_user_presets(&conn, user.id)
                .unwrap()
                .len()
                == 1,
            "Expected user to have 1 preset"
        );
        let delete_result = TableLabelPreset::delete_record(&conn, new_id);
        assert!(delete_result.unwrap() == 1);
        assert!(
            TableLabelPreset::get_user_presets(&conn, user.id)
                .unwrap()
                .is_empty(),
            "Expected user to have no presets"
        );
    }
}
