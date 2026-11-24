use rusqlite::{Connection, Row, params};

use crate::{
    backend::database::tables::{
        setting::table_setting::TableSetting,
        table::{RecordInsert, RecordUpdate},
    },
    shared::models::database::{
        label_preset::model_label_preset::RecordLabelPreset,
        setting::{model_setting::SettingsKey, types_setting::SettingEmpty},
    },
};

impl RecordInsert<RecordLabelPreset> for TableSetting<RecordLabelPreset> {
    fn insert(db: &Connection, record: &RecordLabelPreset) -> Result<i64, rusqlite::Error> {
        db.execute(
            TableSetting::<SettingEmpty>::INSERT_NO_NULLS,
            params![
                record.user_id,
                SettingsKey::UserLabelPreset,
                record.name,
                record.options
            ],
        )?;
        Ok(db.last_insert_rowid())
    }
}

impl RecordUpdate<RecordLabelPreset> for TableSetting<RecordLabelPreset> {
    fn update(db: &Connection, record: &RecordLabelPreset) -> Result<usize, rusqlite::Error> {
        db.execute(
            TableSetting::<SettingEmpty>::UPDATE_ALL,
            params![
                record.user_id,
                SettingsKey::UserLabelPreset,
                record.name,
                record.options,
                record.id,
            ],
        )
    }
}

impl TableSetting<RecordLabelPreset> {
    pub fn get_all(
        db: &Connection,
        user_id: i64,
    ) -> Result<Vec<RecordLabelPreset>, rusqlite::Error> {
        db.prepare(TableSetting::<SettingEmpty>::SELECT_BY_USER_ID_AND_KEY_PRIMARY)?
            .query_map(
                params![user_id, SettingsKey::UserLabelPreset],
                |row: &Row<'_>| {
                    Ok(RecordLabelPreset {
                        id: row.get(0)?,
                        user_id,
                        name: row.get(1)?,
                        options: row.get(2)?,
                    })
                },
            )?
            .collect::<Result<Vec<RecordLabelPreset>, rusqlite::Error>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        backend::database::{
            db::tests::create_test_database,
            tables::{
                setting::table_setting::{self, TableSetting},
                table::{RecordInsert, RecordUpdate},
                user::table_user::{self},
            },
        },
        shared::models::database::label_preset::model_label_preset::{
            LabelOptions, RecordLabelPreset,
        },
    };

    #[test]
    fn suite() {
        let conn = create_test_database();
        table_setting::tests::create_table(&conn);

        let user_id = table_user::tests::any_user(&conn).id;

        assert!(
            TableSetting::<RecordLabelPreset>::get_all(&conn, user_id)
                .unwrap()
                .is_empty()
        );

        let mut test_record = RecordLabelPreset {
            id: 0,
            user_id,
            name: "preset1".to_string(),
            options: LabelOptions::default_preview(),
        };
        let id = TableSetting::<RecordLabelPreset>::insert(&conn, &test_record).unwrap();
        test_record.id = id;

        assert_eq!(
            *TableSetting::<RecordLabelPreset>::get_all(&conn, user_id)
                .unwrap()
                .first()
                .unwrap(),
            test_record
        );

        test_record.name = "new name".to_string();
        assert_eq!(
            TableSetting::<RecordLabelPreset>::update(&conn, &test_record).unwrap(),
            1
        );
        assert_eq!(
            *TableSetting::<RecordLabelPreset>::get_all(&conn, user_id)
                .unwrap()
                .first()
                .unwrap(),
            test_record
        );
    }
}
