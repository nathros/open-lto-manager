use chrono::{DateTime, Local};
use dioxus::fullstack::serde::{Deserialize, Serialize};
use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
};

use super::model_user::{ColourMode, FileTheme, IconTheme};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordUser {
    pub id: i64,
    pub username: String,
    pub description: String,
    pub hash: String,
    pub salt: String,
    pub algorithm: HashAlgorithm,
    pub enabled: bool,
    pub created: DateTime<Local>,
    pub language: i64,
    pub avatar: String,
    pub system_theme: ColourMode,
    pub icon_theme: IconTheme,
    pub file_theme: FileTheme,
    pub accent_colour: String,
}

/*#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordUserWithRoles {
    pub id: i64,
    pub roles: Vec<RecordRole>,
    pub username: String,
    pub description: String,
    pub hash: String,
    pub salt: String,
    pub enabled: bool,
    pub created: DateTime<Local>,
    pub language: i64,
    pub avatar: String,
}*/

#[cfg(feature = "server")]
impl RecordUser {
    pub fn create(username: String, description: String, raw_password: &str) -> RecordUser {
        use crate::{
            backend::crypto::{generate_hash, generate_salt},
            shared::models::database::user::model_user::RecordUserConfig,
        };

        let salt = generate_salt();
        let config = RecordUserConfig::default();
        RecordUser {
            id: config.id,
            username,
            description,
            hash: generate_hash(raw_password, &salt),
            salt,
            algorithm: HashAlgorithm::latest(),
            enabled: config.enabled,
            created: Local::now(),
            language: config.language,
            avatar: config.avatar,
            system_theme: config.system_theme,
            icon_theme: config.icon_theme,
            file_theme: config.file_theme,
            accent_colour: config.accent_colour,
        }
    }
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum HashAlgorithm {
    BallonHashSHA256 = 0,
}

impl HashAlgorithm {
    pub fn latest() -> HashAlgorithm {
        HashAlgorithm::BallonHashSHA256
    }

    pub fn is_latest(&self) -> bool {
        *self == Self::latest()
    }
}

impl From<i64> for HashAlgorithm {
    fn from(value: i64) -> Self {
        match value {
            _ if value == HashAlgorithm::BallonHashSHA256 as i64 => HashAlgorithm::BallonHashSHA256,
            _ => HashAlgorithm::BallonHashSHA256, // Fallback
        }
    }
}

impl ToSql for HashAlgorithm {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok((*self as i64).into())
    }
}

impl FromSql for HashAlgorithm {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::Ok(HashAlgorithm::from(value.as_i64().unwrap_or(0)))
    }
}
