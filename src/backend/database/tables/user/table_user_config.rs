use rusqlite::{Connection, params};

use crate::{
    backend::database::tables::table::{RecordFill, RecordRead, RecordUpdate},
    shared::models::database::user::model_user::RecordUserConfig,
};

use super::table_user::TableUser;

impl RecordRead<RecordUserConfig> for TableUser<RecordUserConfig> {
    fn get(db: &Connection, record_id: i64) -> Result<RecordUserConfig, rusqlite::Error> {
        db.prepare(
            "SELECT
                id,
                username,
                description,
                enabled,
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

impl RecordUpdate<RecordUserConfig> for TableUser<RecordUserConfig> {
    fn update(db: &Connection, record: &RecordUserConfig) -> Result<usize, rusqlite::Error> {
        db.execute(
            "UPDATE user SET
                    username = ?1,
                    description = ?2,
                    enabled = ?3,
                    language = ?4,
                    avatar = ?5,
                    system_theme = ?6,
                    icon_theme = ?7,
                    file_theme = ?8,
                    accent_colour = ?9
                WHERE id = ?10;",
            params![
                record.username,
                record.description,
                record.enabled,
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

impl RecordFill<RecordUserConfig> for TableUser<RecordUserConfig> {
    fn fill(row: &rusqlite::Row<'_>, offset: usize) -> Result<RecordUserConfig, rusqlite::Error> {
        Ok(RecordUserConfig {
            id: row.get(offset)?,
            username: row.get(offset + 1)?,
            description: row.get(offset + 2)?,
            enabled: row.get(offset + 3)?,
            language: row.get(offset + 4)?,
            avatar: row.get(offset + 5)?,
            system_theme: row.get(offset + 6)?,
            icon_theme: row.get(offset + 7)?,
            file_theme: row.get(offset + 8)?,
            accent_colour: row.get(offset + 9)?,
        })
    }
}

impl TableUser<RecordUserConfig> {
    pub fn get_all(db: &Connection) -> Result<Vec<RecordUserConfig>, rusqlite::Error> {
        db.prepare(
            "SELECT
                    id,
                    username,
                    description,
                    enabled,
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
        .collect::<Result<Vec<RecordUserConfig>, rusqlite::Error>>()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        backend::database::{
            db::tests::create_test_database,
            tables::{
                table::{RecordRead, RecordUpdate},
                user::table_user::{TableUser, tests::create_table},
            },
        },
        shared::models::database::user::{
            model_user::RecordUserConfig, model_user_sensitive::RecordUser,
        },
    };

    fn assert_compare_eq(config_user: &RecordUserConfig, test_user: &RecordUser) {
        assert_eq!(config_user.id, test_user.id);
        assert_eq!(config_user.username, test_user.username);
        assert_eq!(config_user.description, test_user.description);
        assert_eq!(config_user.enabled, test_user.enabled);
        assert_eq!(config_user.language, test_user.language);
        assert_eq!(config_user.avatar, test_user.avatar);
        assert_eq!(config_user.system_theme, test_user.system_theme);
        assert_eq!(config_user.icon_theme, test_user.icon_theme);
        assert_eq!(config_user.file_theme, test_user.file_theme);
        assert_eq!(config_user.accent_colour, test_user.accent_colour);
    }

    #[test]
    fn suite() {
        let conn = create_test_database();
        create_table(&conn);

        // Get first found user from complete record
        let test_user = TableUser::<RecordUser>::get_all(&conn)
            .unwrap()
            .first()
            .unwrap()
            .clone();

        // Get same user but as different struct
        let mut config_user = TableUser::<RecordUserConfig>::get(&conn, test_user.id).unwrap();
        // Check same fields are the same
        assert_compare_eq(&config_user, &test_user);

        config_user.enabled = !config_user.enabled;
        config_user.username = "Updated name".to_string();
        assert!(TableUser::<RecordUserConfig>::update(&conn, &config_user).is_ok());

        // Check change applied
        assert_eq!(
            TableUser::<RecordUserConfig>::get(&conn, test_user.id).unwrap(),
            config_user
        );
    }
}
