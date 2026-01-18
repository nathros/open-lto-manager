use chrono::{DateTime, Local};
use dioxus::fullstack::serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
    ToSql,
};

use crate::backend::database::models::{
    model_manufacturer::RecordManufacturer, model_tape_type::RecordTapeType,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordTape {
    pub id: i64,
    pub manufacturer_id: i64,
    pub tape_type_id: i64,
    pub barcode: String,
    pub serial: String,
    pub format: TapeFormat,
    pub worm: bool,
    pub encrypted: bool,
    pub compressed: bool,
    pub used_space: i64,
    pub created: DateTime<Local>,
    pub last_used: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordTapeJoin {
    pub id: i64,
    pub manufacturer: RecordManufacturer,
    pub tape_type: RecordTapeType,
    pub barcode: String,
    pub serial: String,
    pub format: TapeFormat,
    pub worm: bool,
    pub encrypted: bool,
    pub compressed: bool,
    pub used_space: i64,
    pub created: DateTime<Local>,
    pub last_used: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum TapeFormat {
    Tar = 0,
    LTFS = 1,
    // STFS
}

impl From<i64> for TapeFormat {
    fn from(value: i64) -> Self {
        match value {
            0 => TapeFormat::Tar,
            1 => TapeFormat::LTFS,
            _ => TapeFormat::Tar,
        }
    }
}

impl From<TapeFormat> for i64 {
    fn from(value: TapeFormat) -> Self {
        value.into()
    }
}

#[cfg(feature = "server")]
impl ToSql for TapeFormat {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(i64::from(*self).into())
    }
}

#[cfg(feature = "server")]
impl FromSql for TapeFormat {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::Ok(TapeFormat::from(value.as_i64().unwrap())) // FIXME handle out of range
    }
}
