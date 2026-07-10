use dioxus::fullstack::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AppState {
    pub user_name: Option<String>,    // Current username the app is run as
    pub group: String,                // Expected group user to belong to
    pub user_part_of_group: bool, // Most OSs need user added to 'tape' group to access drives, is the user part of this group
    pub ltfs_installed: bool,     // Is ltfs command available
    pub ltfs_version: Option<String>, // Current installed LTFS if any
    pub ltfs_version_latest: Option<String>, // Latest LTFS
    pub ltfs_latest_is_newer: bool, // Is there a newer version of LTFS than what is installed
    pub ltfs_error: Option<String>, // Error message when running: ltfs
    pub mt_installed: bool,       // Is mt command available
    pub mt_error: Option<String>, // Error message when running: mt
    pub platform: String,
    pub distro: String,
    pub cpu_arch: String,
    pub critical_error: bool, // If any critical errors are found then the app cannot start
    pub error_list: Vec<String>, // List of critical errors
    pub pass_count: i32,
    pub warn_count: i32,
    pub err_count: i32,
}
