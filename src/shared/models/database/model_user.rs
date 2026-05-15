use chrono::{DateTime, Local};
use dioxus::fullstack::serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
};

use super::model_role::RecordRole;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordUser {
    pub id: i64,
    pub username: String,
    pub description: String,
    pub hash: String,
    pub salt: String,
    pub enabled: bool,
    pub created: DateTime<Local>,
    pub language: i64,
    pub avatar: String,
    pub system_theme: ColourMode,
    pub icon_theme: String,
    pub fm_theme: String,
    pub accent_colour: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum ColourMode {
    System = 0,
    Dark = 1,
    Light = 2,
}

impl From<i64> for ColourMode {
    fn from(value: i64) -> Self {
        match value {
            _ if value == ColourMode::System as i64 => ColourMode::System,
            _ if value == ColourMode::Dark as i64 => ColourMode::Dark,
            _ if value == ColourMode::Light as i64 => ColourMode::Light,
            _ => ColourMode::Dark, // Fallback
        }
    }
}

#[cfg(feature = "server")]
impl ToSql for ColourMode {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok((*self as i64).into())
    }
}

#[cfg(feature = "server")]
impl FromSql for ColourMode {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::Ok(ColourMode::from(value.as_i64().unwrap_or(0)))
    }
}
