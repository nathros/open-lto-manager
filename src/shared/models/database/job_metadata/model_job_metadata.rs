use dioxus::fullstack::serde::{Deserialize, Serialize};
use enum_iterator::Sequence;
#[cfg(feature = "server")]
use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordJobMetadata {
    pub id: i64,
    pub job_id: i64,
    pub key: JobMetadataKey,
    pub index: i64,
    pub value: String,
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Sequence, Eq, Clone, Copy)]
pub enum JobMetadataKey {
    Undefined = 0,
    FileVirtual = 1,
    FilePhysical = 2,
}

impl From<i64> for JobMetadataKey {
    fn from(value: i64) -> Self {
        match value {
            _ if value == JobMetadataKey::Undefined as i64 => JobMetadataKey::Undefined,
            _ if value == JobMetadataKey::FileVirtual as i64 => JobMetadataKey::FileVirtual,
            _ if value == JobMetadataKey::FilePhysical as i64 => JobMetadataKey::FilePhysical,
            _ => JobMetadataKey::Undefined,
        }
    }
}

#[cfg(feature = "server")]
impl ToSql for JobMetadataKey {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok((*self as i64).into())
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
    use crate::shared::models::{
        database::job_metadata::model_job_metadata::JobMetadataKey,
        test::tests::from_generic_keys_test,
    };

    #[test]
    fn job_type_enum() {}

    #[test]
    fn from_repr_keys() {
        from_generic_keys_test::<JobMetadataKey>(&|s| *s as i64);
    }
}
