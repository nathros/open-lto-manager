use chrono::{DateTime, Local};
use dioxus::fullstack::serde::{Deserialize, Serialize};

use super::model_role::RecordRole;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordUser {
    pub id: i64,
    pub username: String,
    pub description: String,
    pub hash: String,
    pub salt: String,
    pub enabled: bool,
    pub created: DateTime<Local>,
    pub language: i64,
    pub avatar: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordUserWithRoles {
    pub id: i64,
    pub roles: Vec<RecordRole>,
    pub username: String,
    pub description: String,
    pub hash: String,
    pub salt: String,
    pub enabled: bool,
    pub created: DateTime<Local>,
    pub language: i64,
    pub avatar: String,
}
