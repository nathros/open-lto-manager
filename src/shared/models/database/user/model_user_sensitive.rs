use chrono::{DateTime, Local};
use dioxus::fullstack::serde::{Deserialize, Serialize};

use super::model_user::{ColourMode, FileTheme, IconTheme};

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
    pub system_theme: ColourMode,
    pub icon_theme: IconTheme,
    pub file_theme: FileTheme,
    pub accent_colour: String,
}

/*#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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
}*/

#[cfg(feature = "server")]
impl RecordUser {
    pub fn create(username: String, description: String, raw_password: &str) -> RecordUser {
        use crate::{
            backend::crypto::{generate_hash, generate_salt},
            shared::models::database::user::model_user::RecordUserConfig,
        };

        let salt = generate_salt();
        let config = RecordUserConfig::default();
        RecordUser {
            id: config.id,
            username,
            description,
            hash: generate_hash(raw_password, &salt),
            salt,
            enabled: config.enabled,
            created: Local::now(),
            language: config.language,
            avatar: config.avatar,
            system_theme: config.system_theme,
            icon_theme: config.icon_theme,
            file_theme: config.file_theme,
            accent_colour: config.accent_colour,
        }
    }
}
