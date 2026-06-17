use dioxus::prelude::*;

use crate::shared::models::database::user::model_user::RecordUserConfig;

#[get("/api/user/all", _auth: crate::backend::auth::SessionId)]
pub async fn list_users() -> Result<Vec<RecordUserConfig>> {
    use crate::backend::database::{db::DB, tables::user::table_user::TableUser};

    DB.with(|db| match TableUser::<RecordUserConfig>::get_all(db) {
        Ok(records) => Ok(records),
        Err(e) => Err(e)?,
    })
}

#[post("/api/user/update", auth: crate::backend::auth::SessionId)]
pub async fn update_user(user: RecordUserConfig) -> Result<bool> {
    use std::io::Error;

    use crate::backend::{
        auth::Session,
        database::{
            db::DB,
            tables::{table::RecordUpdate, user::table_user::TableUser},
        },
    };
    if let Some(session) = Session::find(&auth)
        && session.user_id == user.id
    // Check is self
    {
        return DB.with(
            |db| match TableUser::<RecordUserConfig>::update(db, &user) {
                Ok(size) => Ok(size > 0),
                Err(e) => Err(e)?,
            },
        );
    }
    Err(Error::other("Cannot edit other user").into())
}
