use dioxus::{
    fullstack::{SetCookie, SetHeader},
    prelude::*,
};

#[get("/api/logout", header: dioxus::fullstack::TypedHeader<dioxus::fullstack::Cookie>)]
pub async fn api_logout() -> Result<SetHeader<SetCookie>> {
    use crate::backend::auth::{SESSION_KEY, Session, SessionId};

    if let Some(session_uuid_str) = header.get(SESSION_KEY) {
        Session::remove(session_uuid_str);
    }

    Ok(SetHeader::new(SessionId::generate_remove_cookie())?)
}
