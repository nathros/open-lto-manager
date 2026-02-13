use chrono::{DateTime, Local};
use dioxus::fullstack::serde::{Deserialize, Serialize};

use super::model_tape::RecordTape;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordFile {
    pub id: i64,
    pub tape_id: Option<i64>, // Foreign key for RecordTape
    pub file_name_virt: String,
    pub file_path_virt: String,
    pub file_name_phy: String,
    pub file_path_phy: String,
    pub file_size: i64,
    pub created: DateTime<Local>,
    pub modified: DateTime<Local>,
    pub crc32: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordFileJoin {
    pub id: i64,
    pub tape: RecordTape,
    pub file_name_virt: String,
    pub file_path_virt: String,
    pub file_name_phy: String,
    pub file_path_phy: String,
    pub file_size: i64,
    pub created: DateTime<Local>,
    pub modified: DateTime<Local>,
    pub crc32: String,
    pub icon: String,
}

impl RecordFile {
    pub fn root_dir() -> RecordFile {
        RecordFile {
            id: 0,
            tape_id: None,
            file_name_virt: "/".to_string(),
            file_path_virt: "/".to_string(),
            file_name_phy: "".to_string(),
            file_path_phy: "".to_string(),
            file_size: 0,
            created: Local::now(),
            modified: Local::now(),
            crc32: "".to_string(),
            icon: "folder~a-root".to_string(),
        }
    }
}
