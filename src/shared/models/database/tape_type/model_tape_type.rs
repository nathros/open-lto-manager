use dioxus::fullstack::serde::{Deserialize, Serialize};

use crate::shared::models::database::tape::model_tape::TapeFormat;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordTapeType {
    pub id: i64,
    pub generation: i64,
    pub description: String,
    pub id_reg: String,
    pub id_worm: String,
    pub native_capacity: i64,
    pub colour_reg: String,
    pub colour_hp: String,
    pub colour_worm_reg: String,
    pub colour_worm_hp: String,
    pub supports_worm: bool,
    pub supports_encryption: bool,
    pub supports_ltfs: bool,
}

impl RecordTapeType {
    pub fn get_supported_format(&self) -> Vec<TapeFormat> {
        let mut supports = vec![TapeFormat::Tar];
        if self.supports_ltfs {
            supports.push(TapeFormat::LTFS);
        }
        supports
    }
}

impl Default for RecordTapeType {
    fn default() -> Self {
        Self {
            id: 0,
            generation: 0,
            description: "".to_string(),
            id_reg: "".to_string(),
            id_worm: "".to_string(),
            native_capacity: 0,
            colour_reg: "".to_string(),
            colour_hp: "".to_string(),
            colour_worm_reg: "".to_string(),
            colour_worm_hp: "".to_string(),
            supports_worm: true,
            supports_encryption: true,
            supports_ltfs: true,
        }
    }
}
