use dioxus::fullstack::serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
};

#[cfg(feature = "server")]
use super::model_user_sensitive::RecordUser;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordUserConfig {
    pub id: i64,
    pub username: String,
    pub description: String,
    pub enabled: bool,
    pub language: i64,
    pub avatar: String,
    pub system_theme: ColourMode,
    pub icon_theme: IconTheme,
    pub file_theme: FileTheme,
    pub accent_colour: String,
}

#[cfg(feature = "server")]
impl From<RecordUser> for RecordUserConfig {
    fn from(value: RecordUser) -> Self {
        RecordUserConfig {
            id: value.id,
            username: value.username,
            description: value.description,
            enabled: value.enabled,
            language: value.language,
            avatar: value.avatar,
            system_theme: value.system_theme,
            icon_theme: value.icon_theme,
            file_theme: value.file_theme,
            accent_colour: value.accent_colour,
        }
    }
}

impl Default for RecordUserConfig {
    fn default() -> Self {
        Self {
            id: 0,
            username: "default".to_string(),
            description: "default".to_string(),
            enabled: true,
            language: 0,
            avatar: "".to_string(),
            system_theme: ColourMode::System,
            icon_theme: IconTheme::Tabler,
            file_theme: FileTheme::Breeze,
            accent_colour: "#1677ff".to_string(),
        }
    }
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

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum IconTheme {
    Tabler = 0,
    Remix = 1,
}

impl From<i64> for IconTheme {
    fn from(value: i64) -> Self {
        match value {
            _ if value == IconTheme::Tabler as i64 => IconTheme::Tabler,
            _ if value == IconTheme::Remix as i64 => IconTheme::Remix,
            _ => IconTheme::Tabler, // Fallback
        }
    }
}

#[cfg(feature = "server")]
impl ToSql for IconTheme {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok((*self as i64).into())
    }
}

#[cfg(feature = "server")]
impl FromSql for IconTheme {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::Ok(IconTheme::from(value.as_i64().unwrap_or(0)))
    }
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum FileTheme {
    Breeze = 0,
    Kora = 1,
}

impl From<i64> for FileTheme {
    fn from(value: i64) -> Self {
        match value {
            _ if value == FileTheme::Breeze as i64 => FileTheme::Breeze,
            _ if value == FileTheme::Kora as i64 => FileTheme::Kora,
            _ => FileTheme::Breeze, // Fallback
        }
    }
}

#[cfg(feature = "server")]
impl ToSql for FileTheme {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok((*self as i64).into())
    }
}

#[cfg(feature = "server")]
impl FromSql for FileTheme {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::Ok(FileTheme::from(value.as_i64().unwrap_or(0)))
    }
}
