use dioxus::fullstack::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AppState {
    pub user_name: String,               // Current username the app is run as
    pub user_name_error: Option<String>, // Was there an error in retrieving the username
    pub part_tape_group: bool, // Most OSs need user added to 'tape' group to access drives, is the user part of this group
    pub ltfs_installed: bool,  // Is ltfs command available
    pub ltfs_error: Option<String>, // Error message when running: ltfs
    pub platform: String,
    pub cpu_arch: String,
    pub critical_error: bool,
    pub error_list: Vec<String>,
}
