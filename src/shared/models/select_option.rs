use dioxus::fullstack::serde::{Deserialize, Serialize};

use crate::shared::models::database::tape_type::model_tape_type::RecordTapeTypeLabel;

use super::database::{
    manufacturer::model_manufacturer::RecordManufacturer,
    tape_type::model_tape_type::RecordTapeType,
};

#[derive(Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct SelectOption {
    pub id: i64,
    pub label: String,
}

impl From<RecordManufacturer> for SelectOption {
    fn from(value: RecordManufacturer) -> Self {
        SelectOption {
            id: value.id,
            label: value.name,
        }
    }
}

impl From<RecordTapeType> for SelectOption {
    fn from(value: RecordTapeType) -> Self {
        SelectOption {
            id: value.id,
            label: value.description,
        }
    }
}

impl From<RecordTapeTypeLabel> for SelectOption {
    fn from(value: RecordTapeTypeLabel) -> Self {
        SelectOption {
            id: value.id,
            label: value.description,
        }
    }
}

pub fn vec_into<T, U>(v: Vec<T>) -> Vec<U>
where
    T: Into<U>,
{
    v.into_iter().map(Into::into).collect()
}
