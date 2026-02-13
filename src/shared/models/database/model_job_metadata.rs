use dioxus::fullstack::serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
};

use crate::shared::models::database::model_job::RecordJob;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordJobMetadata {
    pub id: i64,
    pub job_id: i64,
    pub key: JobMetadataKey,
    pub index: i64,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordJobMetadataJoin {
    pub id: i64,
    pub job: RecordJob,
    pub key: JobMetadataKey,
    pub index: i64,
    pub value: String,
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum JobMetadataKey {
    Undefined = 0,
    FileVirtual = 1,
    FilePhysical = 2,
}

impl From<i64> for JobMetadataKey {
    fn from(value: i64) -> Self {
        match value {
            0 => JobMetadataKey::Undefined,
            1 => JobMetadataKey::FileVirtual,
            2 => JobMetadataKey::FilePhysical,
            _ => JobMetadataKey::Undefined,
        }
    }
}

impl From<JobMetadataKey> for i64 {
    fn from(value: JobMetadataKey) -> Self {
        value as i64
    }
}

#[cfg(feature = "server")]
impl ToSql for JobMetadataKey {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(i64::from(*self).into())
    }
}

#[cfg(feature = "server")]
impl FromSql for JobMetadataKey {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::Ok(JobMetadataKey::from(value.as_i64().unwrap_or(0)))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn job_type_enum() {}
}
