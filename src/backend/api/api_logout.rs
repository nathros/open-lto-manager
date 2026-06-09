use dioxus::{
    fullstack::{Cookie, SetCookie, SetHeader},
    prelude::*,
};

#[get("/api/logout", header: dioxus::fullstack::TypedHeader<Cookie>)]
pub async fn api_logout() -> Result<SetHeader<SetCookie>> {
    use dioxus::fullstack::SetHeader;

    use crate::backend::auth::{SESSION_KEY, Session, SessionId};

    #[cfg(feature = "slow_server")]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    if let Some(session_uuid_str) = header.get(SESSION_KEY) {
        Session::remove(session_uuid_str);
    }

    Ok(SetHeader::new(SessionId::generate_remove_cookie())?)
}
