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

impl From<JobType> for &str {
    fn from(value: JobType) -> &'static str {
        match value {
            JobType::Backup => "Backup",
            JobType::Restore => "Restore",
            JobType::Delete => "Delete",
            JobType::Duplicate => "Duplicate",
        }
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
            0 => JobStatus::Unknown,
            1 => JobStatus::Pending,
            2 => JobStatus::InProgress,
            3 => JobStatus::Stopped,
            4 => JobStatus::Interrupted,
            5 => JobStatus::Completed,
            6 => JobStatus::Error,
            _ => JobStatus::Unknown,
        }
    }
}

impl From<JobStatus> for i64 {
    fn from(value: JobStatus) -> Self {
        value as i64
    }
}

impl From<JobStatus> for &str {
    fn from(value: JobStatus) -> &'static str {
        match value {
            JobStatus::Unknown => "Unknown",
            JobStatus::Pending => "Pending",
            JobStatus::InProgress => "InProgress",
            JobStatus::Stopped => "Stopped",
            JobStatus::Interrupted => "Interrupted",
            JobStatus::Completed => "Completed",
            JobStatus::Error => "Error",
        }
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
