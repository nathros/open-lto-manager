use chrono::{DateTime, Local};
use dioxus::fullstack::serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
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
    pub encryption_type: EncryptionType,
    pub encryption_sw: SoftwareEncryptionType,
    pub encryption_hw: HardwareEncryptionType,
    pub compressed: bool,
    pub used_space: i64,
    pub created: DateTime<Local>,
    pub last_used: DateTime<Local>,
}

impl Default for RecordTape {
    fn default() -> Self {
        Self {
            id: 0,
            manufacturer_id: 0,
            tape_type_id: 0,
            barcode: "".to_string(),
            serial: "".to_string(),
            format: TapeFormat::Tar,
            worm: false,
            encryption_type: EncryptionType::None,
            encryption_sw: SoftwareEncryptionType::None,
            encryption_hw: HardwareEncryptionType::None,
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
    #[allow(clippy::upper_case_acronyms)]
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
        FromSqlResult::Ok(TapeFormat::from(value.as_i64().unwrap_or(0)))
    }
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum EncryptionType {
    None = 0,
    Software = 1,
    Hardware = 2,
}

impl From<i64> for EncryptionType {
    fn from(value: i64) -> Self {
        match value {
            _ if value == EncryptionType::None as i64 => EncryptionType::None,
            _ if value == EncryptionType::Software as i64 => EncryptionType::Software,
            _ if value == EncryptionType::Hardware as i64 => EncryptionType::Hardware,
            _ => EncryptionType::None,
        }
    }
}

#[cfg(feature = "server")]
impl ToSql for EncryptionType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok((*self as i64).into())
    }
}

#[cfg(feature = "server")]
impl FromSql for EncryptionType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::Ok(EncryptionType::from(value.as_i64().unwrap_or(0)))
    }
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum SoftwareEncryptionType {
    None = 0,
    Test = 1,
}

impl From<i64> for SoftwareEncryptionType {
    fn from(value: i64) -> Self {
        match value {
            _ if value == SoftwareEncryptionType::None as i64 => SoftwareEncryptionType::None,
            _ if value == SoftwareEncryptionType::Test as i64 => SoftwareEncryptionType::Test,
            _ => SoftwareEncryptionType::None, // Fallback
        }
    }
}

#[cfg(feature = "server")]
impl ToSql for SoftwareEncryptionType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok((*self as i64).into())
    }
}

#[cfg(feature = "server")]
impl FromSql for SoftwareEncryptionType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::Ok(SoftwareEncryptionType::from(value.as_i64().unwrap_or(0)))
    }
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum HardwareEncryptionType {
    None = 0,
    Test = 1,
}

impl From<i64> for HardwareEncryptionType {
    fn from(value: i64) -> Self {
        match value {
            _ if value == HardwareEncryptionType::None as i64 => HardwareEncryptionType::None,
            _ if value == HardwareEncryptionType::Test as i64 => HardwareEncryptionType::Test,
            _ => HardwareEncryptionType::None, // Fallback
        }
    }
}

#[cfg(feature = "server")]
impl ToSql for HardwareEncryptionType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok((*self as i64).into())
    }
}

#[cfg(feature = "server")]
impl FromSql for HardwareEncryptionType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::Ok(HardwareEncryptionType::from(value.as_i64().unwrap_or(0)))
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::types::ValueRef;

    use crate::shared::models::database::tape::model_tape::TapeFormat;

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

        let ltfs_str = format!("{:?}", TapeFormat::LTFS);
        assert_eq!(ltfs_str, "LTFS");

        let sql_tar = ValueRef::Integer(to_int_tar);
        assert_eq!(sql_tar.as_i64().unwrap(), 0i64);
    }
}
