use std::marker::PhantomData;

use rusqlite::{Connection, Row, params};

use crate::{
    backend::database::tables::table::{
        RecordDelete, RecordFill, RecordInsert, RecordRead, RecordUpdate, TableCreate, TableUpdate,
    },
    shared::models::database::label_preset::model_label_preset::RecordLabelPreset,
};

pub struct TableLabelPreset<T = RecordLabelPreset> {
    phantom: PhantomData<T>,
}

impl TableCreate<RecordLabelPreset> for TableLabelPreset<RecordLabelPreset> {
    fn create_table(db: &Connection) -> Result<bool, rusqlite::Error> {
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
}

impl TableUpdate<RecordLabelPreset> for TableLabelPreset<RecordLabelPreset> {
    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, rusqlite::Error> {
        Ok(false)
    }
}

impl RecordRead<RecordLabelPreset> for TableLabelPreset<RecordLabelPreset> {
    fn get(db: &Connection, record_id: i64) -> Result<RecordLabelPreset, rusqlite::Error> {
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
}

impl RecordInsert<RecordLabelPreset> for TableLabelPreset<RecordLabelPreset> {
    fn insert(db: &Connection, record: &RecordLabelPreset) -> Result<i64, rusqlite::Error> {
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
}

impl RecordUpdate<RecordLabelPreset> for TableLabelPreset<RecordLabelPreset> {
    fn update(db: &Connection, record: &RecordLabelPreset) -> Result<usize, rusqlite::Error> {
        db.execute(
            "UPDATE label_preset SET
                    user_id = ?1,
                    name = ?2,
                    settings = ?3
                WHERE id = ?4;",
            params![record.user_id, record.name, record.options, record.id],
        )
    }
}

impl RecordDelete<RecordLabelPreset> for TableLabelPreset<RecordLabelPreset> {
    fn delete(db: &Connection, record_id: i64) -> Result<usize, rusqlite::Error> {
        db.execute(
            "DELETE FROM label_preset WHERE id = ?1;",
            params![record_id],
        )
    }
}

impl RecordFill<RecordLabelPreset> for TableLabelPreset<RecordLabelPreset> {
    fn fill(row: &Row<'_>, offset: usize) -> Result<RecordLabelPreset, rusqlite::Error> {
        Ok(RecordLabelPreset {
            id: row.get(0)?,
            user_id: row.get(offset + 1)?,
            name: row.get(offset + 2)?,
            options: row.get(offset + 3)?,
        })
    }
}

impl TableLabelPreset<RecordLabelPreset> {
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
        backend::database::{
            db::tests::create_test_database,
            tables::{
                label_preset::table_label_preset::TableLabelPreset,
                table::{RecordDelete, RecordInsert, RecordUpdate, TableCreate},
                user::{self, table_user::TableUser},
            },
        },
        shared::models::database::{
            label_preset::model_label_preset::{LabelOptions, RecordLabelPreset},
            user::model_user::RecordUserConfig,
        },
    };

    fn create() -> rusqlite::Connection {
        let conn = create_test_database();
        // TableLabelPreset depends on TableUser, so this must be created first
        user::table_user::tests::create_table(&conn);

        assert!(
            !conn.table_exists(None, "label_preset").unwrap(),
            "New table should be empty"
        );
        assert!(
            TableLabelPreset::create_table(&conn).is_ok(),
            "Failed to create table"
        );
        assert!(
            conn.table_exists(None, "label_preset").unwrap(),
            "create_table() reported Ok but table does not exist"
        );
        conn
    }

    #[test]
    fn create_table() {
        create();
    }

    #[test]
    fn insert_and_update() {
        let conn = create();
        let users = TableUser::<RecordUserConfig>::get_all(&conn).unwrap();
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
        assert!(TableLabelPreset::insert(&conn, &preset).is_ok());
        let user_presets = TableLabelPreset::get_user_presets(&conn, user.id).unwrap();
        let user_preset = user_presets.first().unwrap();
        preset.id = user_preset.id; // Before insert id cannot be known
        assert_eq!(preset, *user_preset, "Expected new preset to be the same");

        // Update
        preset.name = "new_name".to_string();
        preset.options.stroke_inner = 99_f64;
        assert!(TableLabelPreset::update(&conn, &preset).is_ok());
        let user_presets = TableLabelPreset::get_user_presets(&conn, user.id).unwrap();
        let user_preset = user_presets.first().unwrap();
        assert_eq!(preset, *user_preset, "Expected new preset to be the same");
    }

    #[test]
    fn delete() {
        let conn = create();
        let users = TableUser::<RecordUserConfig>::get_all(&conn).unwrap();
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
        let new_id = TableLabelPreset::insert(&conn, &preset).unwrap();
        assert!(
            TableLabelPreset::get_user_presets(&conn, user.id)
                .unwrap()
                .len()
                == 1,
            "Expected user to have 1 preset"
        );
        let delete_result = TableLabelPreset::delete(&conn, new_id);
        assert!(delete_result.unwrap() == 1);
        assert!(
            TableLabelPreset::get_user_presets(&conn, user.id)
                .unwrap()
                .is_empty(),
            "Expected user to have no presets"
        );
    }
}
