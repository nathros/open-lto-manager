use dioxus::fullstack::serde::{Deserialize, Serialize};
use enum_iterator::Sequence;
use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
};

use crate::shared::models::database::setting::types_setting::SettingTableVersion;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordMisc<T> {
    pub key: SettingsKey,
    pub data: T,
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Sequence, Clone, Copy)]
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

// Helpers
impl From<i64> for RecordMisc<SettingTableVersion> {
    fn from(data: i64) -> Self {
        RecordMisc::<SettingTableVersion> {
            key: SettingsKey::Version,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::models::{
        database::setting::model_setting::SettingsKey, test::tests::from_generic_keys_test,
    };

    #[test]
    fn from_repr_keys() {
        from_generic_keys_test::<SettingsKey>(&|s| *s as i64);
    }
}
