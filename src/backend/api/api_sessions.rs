use chrono::TimeZone;
use dioxus::{
    fullstack::serde::{Deserialize, Serialize},
    prelude::*,
};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct SessionInfo {
    pub uuid: String,
    pub username: String,
    pub expiry: String,
}

#[get("/api/sessions", _auth: crate::backend::auth::SessionId)]
pub async fn list_sessions() -> Result<Vec<SessionInfo>> {
    use crate::{
        backend::{
            auth::Session,
            database::{db::DB, tables::user::table_user::TableUser},
        },
        shared::models::database::user::model_user::RecordUserConfig,
    };
    use chrono::Local;

    DB.with(|db| match TableUser::<RecordUserConfig>::get_all(db) {
        Ok(users) => {
            let mut results = vec![];
            for (session_id, session) in Session::current() {
                let username = users
                    .iter()
                    .find(|u| u.id == session.user_id)
                    .map_or_else(|| "??".to_string(), |f| f.username.clone());

                results.push(SessionInfo {
                    uuid: session_id.0,
                    username,
                    expiry: Local
                        .timestamp_millis_opt(session.expire)
                        .unwrap()
                        .to_string(),
                });
            }
            Ok(results)
        }
        Err(e) => Err(e)?,
    })
}
