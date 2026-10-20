use chrono::{DateTime, Local};
use dioxus::fullstack::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordFile {
    pub id: i64,
    pub parent_id: i64,       // Foreign key for self, root dir is 0
    pub user_id: i64,         // Foreign key for RecordUser
    pub tape_id: Option<i64>, // Foreign key for RecordTape
    pub file_name_virt: String,
    pub file_path_virt: Option<String>, // Use None for not directory
    pub file_name_phy: String,
    pub file_path_phy: String,
    pub file_size: i64,
    pub created: DateTime<Local>,
    pub modified: DateTime<Local>,
    pub hash: String,
    pub icon: String,
}

impl RecordFile {
    pub fn root_dir(user_id: i64) -> RecordFile {
        RecordFile {
            id: 0,
            parent_id: 0,
            user_id,
            tape_id: None,
            file_name_virt: "/".to_string(),
            file_path_virt: Some("/".to_string()),
            file_name_phy: "".to_string(),
            file_path_phy: "".to_string(),
            file_size: 0,
            created: Local::now(),
            modified: Local::now(),
            hash: "".to_string(),
            icon: "folder-root".to_string(),
        }
    }
}
