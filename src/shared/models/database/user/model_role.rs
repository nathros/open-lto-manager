use dioxus::fullstack::serde::{Deserialize, Serialize};

use super::model_user::RecordUserConfig;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[allow(dead_code)] // FIXME
pub struct RecordRole {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: String,
    pub permission1: i64,
    pub permission2: i64,
    pub permission3: i64,
    pub permission4: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[allow(dead_code)] // FIXME
pub struct RecordRoleJoin {
    pub id: i64,
    pub user: RecordUserConfig,
    pub name: String,
    pub description: String,
    pub permission1: i64,
    pub permission2: i64,
    pub permission3: i64,
    pub permission4: i64,
}
