use dioxus::fullstack::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RecordTapeType {
    pub id: i64,
    pub generation: String,
    pub id_reg: String,
    pub id_worm: String,
    pub native_capacity: i64,
    pub colour_reg: String,
    pub colour_hp: String,
    pub colour_worm_reg: String,
    pub colour_worm_hp: String,
}
