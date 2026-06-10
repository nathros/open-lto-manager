use dioxus::fullstack::serde::{Deserialize, Serialize};
use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordMisc<T> {
    pub key: SettingsKey,
    pub data: T,
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum SettingsKey {
    None = 0,
    Version = 1,
}

impl From<i64> for SettingsKey {
    fn from(value: i64) -> Self {
        match value {
            _ if value == SettingsKey::None as i64 => SettingsKey::None,
            _ if value == SettingsKey::Version as i64 => SettingsKey::Version,
            _ => SettingsKey::None,
        }
    }
}

impl ToSql for SettingsKey {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok((*self as i64).into())
    }
}

impl FromSql for SettingsKey {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::Ok(SettingsKey::from(value.as_i64().unwrap_or(0)))
    }
}
