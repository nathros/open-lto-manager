use dioxus::{
    fullstack::{SetCookie, SetHeader},
    prelude::*,
};

use crate::shared::models::database::user::model_user::RecordUserConfig;

#[post("/api/login")]
pub async fn api_login(username: String, password: String) -> Result<SetHeader<SetCookie>> {
    use crate::backend::{
        auth::Session,
        database::{db::DB, tables::user::table_user::TableUser},
    };

    let cookie = DB.with(|db| match TableUser::get_by_username(db, username) {
        Ok(user) => {
            if let Some(session_id) = Session::new_and_add(user, password) {
                return Some(session_id.generate_set_cookie());
            }
            error!("Failed to validate user login");
            None
        }
        Err(e) => {
            error!("Failed to query user login {}", e);
            None
        }
    });

    if let Some(cookie) = cookie {
        return Ok(SetHeader::new(cookie)?);
    }

    HttpError::unauthorized("Invalid username or password")?
}

#[get("/api/current_user", header: dioxus::fullstack::TypedHeader<dioxus::fullstack::Cookie>)]
pub async fn api_current_user() -> Result<Option<RecordUserConfig>> {
    use crate::backend::{
        auth::{SESSION_KEY, Session, SessionId},
        database::{
            db::DB,
            tables::{table::RecordRead, user::table_user::TableUser},
        },
    };

    if let Some(session_uuid_str) = header.get(SESSION_KEY)
        && let Some(session) = Session::find(&SessionId(session_uuid_str.to_string()))
    {
        return DB.with(
            |db| match TableUser::<RecordUserConfig>::get(db, session.user_id) {
                Ok(user) => Ok(Some(user)),
                Err(e) => {
                    error!("Failed to check current user: {}", e);
                    Ok(None)
                }
            },
        );
    }
    Ok(None)
}

#[cfg(all(feature = "auto_login", debug_assertions))]
#[post("/api/login_bypass")]
pub async fn api_login_bypass() -> Result<SetHeader<SetCookie>> {
    use crate::backend::{
        auth::Session,
        database::{
            db::DB,
            tables::{table::RecordRead, user::table_user::TableUser},
        },
    };

    let cookie = DB.with(|db| match TableUser::get(db, 1) {
        Ok(user) => {
            if let Some(session_id) = Session::new_and_add_bypass(user) {
                return Some(session_id.generate_set_cookie());
            }
            error!("Failed to validate user login bypass");
            None
        }
        Err(e) => {
            error!("Failed to query user login bypass {}", e);
            None
        }
    });

    if let Some(cookie) = cookie {
        return Ok(SetHeader::new(cookie)?);
    }

    HttpError::unauthorized("Invalid username or password")?
}
