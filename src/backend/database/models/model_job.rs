use chrono::{DateTime, Local};
use dioxus::fullstack::serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
    ToSql,
};

use super::model_user::RecordUser;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordJob {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub job_type: JobType,
    pub job_status: JobStatus,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordJobJoin {
    pub id: i64,
    pub user: RecordUser,
    pub name: String,
    pub job_type: JobType,
    pub job_status: JobStatus,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub comment: String,
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum JobType {
    Backup = 0,
    Restore = 1,
    Delete = 2,
    Duplicate = 3,
}

impl From<i64> for JobType {
    fn from(value: i64) -> Self {
        match value {
            0 => JobType::Backup,
            1 => JobType::Restore,
            2 => JobType::Delete,
            3 => JobType::Duplicate,
            _ => JobType::Backup,
        }
    }
}

impl From<JobType> for i64 {
    fn from(value: JobType) -> Self {
        value as i64
    }
}

#[cfg(feature = "server")]
impl ToSql for JobType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(i64::from(*self).into())
    }
}

#[cfg(feature = "server")]
impl FromSql for JobType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::Ok(JobType::from(value.as_i64().unwrap()))
    }
}

#[repr(i64)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum JobStatus {
    Unknown = 0,
    Pending = 1,
    InProgress = 2,
    Stopped = 3,
    Interrupted = 4,
    Error = 5,
}

impl From<i64> for JobStatus {
    fn from(value: i64) -> Self {
        match value {
            0 => JobStatus::Unknown,
            1 => JobStatus::Pending,
            2 => JobStatus::InProgress,
            3 => JobStatus::Stopped,
            4 => JobStatus::Interrupted,
            5 => JobStatus::Error,
            _ => JobStatus::Unknown,
        }
    }
}

impl From<JobStatus> for i64 {
    fn from(value: JobStatus) -> Self {
        value as i64
    }
}

#[cfg(feature = "server")]
impl ToSql for JobStatus {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(i64::from(*self).into())
    }
}

#[cfg(feature = "server")]
impl FromSql for JobStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::Ok(JobStatus::from(value.as_i64().unwrap()))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn job_type_enum() {}

    #[test]
    fn job_status_enum() {}
}
