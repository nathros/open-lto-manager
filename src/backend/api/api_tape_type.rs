use crate::backend::database::models::model_tape_type::RecordTapeType;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::backend::database::tables::table_tape_type::TableTapeType;

#[get("/api/tape_type")]
pub async fn list_type_type() -> Result<Vec<RecordTapeType>> {
    use crate::backend::database::db::DB;
    DB.with(|db| Ok(TableTapeType::get_all(db).unwrap()))
}
