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
    use crate::backend::{
        auth::Session,
        database::{db::DB, tables::table_user::TableUser},
    };
    use chrono::Local;

    #[cfg(feature = "slow_server")]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    DB.with(|db| match TableUser::get_all(db) {
        Ok(users) => {
            let mut results = vec![];
            for (session_id, session) in Session::current() {
                let username = users
                    .iter()
                    .find(|u| u.id == session.user_id)
                    .map_or_else(|| "??".to_string(), |f| f.username.clone());

                results.push(SessionInfo {
                    uuid: session_id.get_uuid().to_string(),
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
