use dioxus::fullstack::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FileView {
    pub is_dir: bool,
    pub file_name: String,
    pub size: u64,
}
