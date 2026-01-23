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

impl RecordTape {
    pub fn blank() -> RecordTape {
        RecordTape {
            id: 0,
            manufacturer_id: 0,
            tape_type_id: 0,
            barcode: "".to_string(),
            serial: "".to_string(),
            format: TapeFormat::Tar,
            worm: false,
            encrypted: false,
            compressed: false,
            used_space: 0,
            created: Local::now(),
            last_used: Local::now(),
        }
    }
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum TapeFormat {
    Tar = 0,
    LTFS = 1,
    // STFS TODO add support
}

impl From<i64> for TapeFormat {
    fn from(value: i64) -> Self {
        match value {
            0 => TapeFormat::Tar,
            1 => TapeFormat::LTFS,
            _ => TapeFormat::Tar, // Fallback
        }
    }
}

impl Into<&str> for TapeFormat {
    fn into(self) -> &'static str {
        match self {
            TapeFormat::Tar => "Tar",
            TapeFormat::LTFS => "LTFS",
        }
    }
}

impl From<TapeFormat> for i64 {
    fn from(value: TapeFormat) -> Self {
        value as i64 // Do not use value.into() will cause stack overflow
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
        FromSqlResult::Ok(TapeFormat::from(value.as_i64().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::types::ValueRef;

    use crate::backend::database::models::model_tape::TapeFormat;

    #[test]
    fn table_format_enum() {
        let enum_int_a: i64 = 0;
        let enum_int_b: i64 = 1;
        let enum_int_c: i64 = 99; // Out of range fallback
        assert_eq!(TapeFormat::Tar, TapeFormat::from(enum_int_a));
        assert_eq!(TapeFormat::LTFS, TapeFormat::from(enum_int_b));
        assert_eq!(TapeFormat::Tar, TapeFormat::from(enum_int_c));

        let to_int_tar: i64 = TapeFormat::Tar.into();
        assert_eq!(to_int_tar, 0i64);
        let to_int_ltfs: i64 = TapeFormat::LTFS.into();
        assert_eq!(to_int_ltfs, 1i64);

        let ltfs_str = <TapeFormat as Into<&str>>::into(TapeFormat::LTFS);
        assert_eq!(ltfs_str, "LTFS");

        let sql_tar = ValueRef::Integer(to_int_tar);
        assert_eq!(sql_tar.as_i64().unwrap(), 0i64);
    }
}
