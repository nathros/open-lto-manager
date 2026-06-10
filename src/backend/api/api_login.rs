use dioxus::{
    fullstack::{SetCookie, SetHeader},
    prelude::*,
};

#[post("/api/login")]
pub async fn api_login(username: String, password: String) -> Result<SetHeader<SetCookie>> {
    use crate::backend::{
        auth::Session,
        database::{db::DB, tables::table_user::TableUser},
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
