use dioxus::fullstack::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AppState {
    pub user_name: String,
    pub platform: String,
    pub cpu_arch: String,
    pub critical_error: bool,
    pub error_list: Vec<String>,
}
