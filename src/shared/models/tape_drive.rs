use dioxus::fullstack::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TapeDrive {
    pub dev: String,
    pub manufacturer: String,
}
