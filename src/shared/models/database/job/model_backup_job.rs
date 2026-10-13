use chrono::{DateTime, Local};
use dioxus::fullstack::serde::{Deserialize, Serialize};

use crate::shared::models::database::job::model_job::{JobStatus, JobType};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordBackupJob {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub job_type: JobType,
    pub job_status: JobStatus,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub comment: String,
    pub files: Vec<String>,
}

impl RecordBackupJob {
    pub fn blank() -> RecordBackupJob {
        RecordBackupJob {
            id: 0,
            user_id: 0,
            name: "".to_string(),
            job_type: JobType::Backup,
            job_status: JobStatus::Unknown,
            start_time: Local::now(),
            end_time: Local::now(),
            comment: "".to_string(),
            files: vec![],
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {}
}
