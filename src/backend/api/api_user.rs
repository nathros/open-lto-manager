use dioxus::prelude::*;

use crate::shared::models::database::user::model_user::RecordUserConfig;

#[get("/api/user/all")]
pub async fn list_users() -> Result<Vec<RecordUserConfig>> {
    use crate::backend::database::{db::DB, tables::user::table_user::TableUser};

    DB.with(|db| match TableUser::<RecordUserConfig>::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}
