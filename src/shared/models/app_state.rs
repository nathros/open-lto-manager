use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AppState {
    pub critical_error: bool,
    pub error_list: Vec<String>,
}
