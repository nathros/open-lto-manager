use dioxus::prelude::*;

use crate::shared::models::database::model_user::RecordUser;

#[get("/api/user/all")]
pub async fn list_users() -> Result<Vec<RecordUser>> {
    use crate::backend::database::{db::DB, tables::table_user::TableUser};

    #[cfg(feature = "slow_server")]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    DB.with(|db| match TableUser::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}
