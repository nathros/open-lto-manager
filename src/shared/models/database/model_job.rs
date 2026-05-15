use chrono::{DateTime, Local};
use dioxus::fullstack::serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef},
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

impl RecordJob {
    pub fn blank(job_type: JobType) -> Self {
        Self {
            id: 0,
            user_id: 1,
            name: "".to_string(),
            job_type,
            job_status: JobStatus::Unknown,
            start_time: Local::now(),
            end_time: Local::now(),
            comment: "".to_string(),
        }
    }
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
            _ if value == JobType::Backup as i64 => JobType::Backup,
            _ if value == JobType::Restore as i64 => JobType::Restore,
            _ if value == JobType::Delete as i64 => JobType::Delete,
            _ if value == JobType::Duplicate as i64 => JobType::Duplicate,
            _ if value == JobType::Backup as i64 => JobType::Backup,
            _ => JobType::Backup,
        }
    }
}

#[cfg(feature = "server")]
impl ToSql for JobType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok((*self as i64).into())
    }
}

#[cfg(feature = "server")]
impl FromSql for JobType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::Ok(JobType::from(value.as_i64().unwrap_or(0)))
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
    Completed = 5,
    Error = 6,
}

impl From<i64> for JobStatus {
    fn from(value: i64) -> Self {
        match value {
            _ if value == JobStatus::Unknown as i64 => JobStatus::Unknown,
            _ if value == JobStatus::Pending as i64 => JobStatus::Pending,
            _ if value == JobStatus::InProgress as i64 => JobStatus::InProgress,
            _ if value == JobStatus::Stopped as i64 => JobStatus::Stopped,
            _ if value == JobStatus::Interrupted as i64 => JobStatus::Interrupted,
            _ if value == JobStatus::Completed as i64 => JobStatus::Completed,
            _ if value == JobStatus::Error as i64 => JobStatus::Error,
            _ if value == JobStatus::Unknown as i64 => JobStatus::Unknown,
            _ => JobStatus::Unknown,
        }
    }
}

#[cfg(feature = "server")]
impl ToSql for JobStatus {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok((*self as i64).into())
    }
}

#[cfg(feature = "server")]
impl FromSql for JobStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        FromSqlResult::Ok(JobStatus::from(value.as_i64().unwrap_or(0)))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn job_type_enum() {}

    #[test]
    fn job_status_enum() {}
}
