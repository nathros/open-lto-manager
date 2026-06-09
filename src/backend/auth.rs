use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex, RwLock},
};

use axum::{
    body::Body,
    http::{self, HeaderMap, HeaderValue},
    middleware::Next,
    response::Response,
};
use chrono::Local;
use dioxus::fullstack::extract::FromRequestParts;
use http::request::Parts;
use reqwest::header::COOKIE;
use tracing::{trace, warn};
use uuid::Uuid;

use crate::shared::models::database::model_user::RecordUser;

use super::crypto::validate_password;

pub const SESSION_KEY: &str = "session";

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SessionId(Uuid); // Used to validate sessions

#[non_exhaustive]
#[derive(Clone)]
pub struct Session {
    pub user_id: i64,
    pub expire: i64,
}

type SessionMap = RwLock<HashMap<SessionId, Mutex<Session>>>;
static SESSIONS: LazyLock<SessionMap> = LazyLock::new(|| RwLock::new(HashMap::new()));

impl<S> FromRequestParts<S> for SessionId
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(req: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        //let session = Session::from_request_parts(req, _state).await?;

        //info!("Called {:?}", req);
        if let Some(session_id_str) = SessionId::get_session_id(&req.headers)
            && let Ok(session_id) = Uuid::parse_str(session_id_str)
            && let Some(session) = Session::find(&SessionId(session_id))
        {
            if session.expired() {
                warn!(
                    "Session {} has expired for user {}",
                    session_id, session.user_id
                );
                return Err((http::StatusCode::UNAUTHORIZED, "Session expired"));
            } else {
                return Ok(SessionId(session_id)); // Session exists and is value
            }
        }

        Err((http::StatusCode::UNAUTHORIZED, "Not authorised"))
    }
}

impl SessionId {
    pub async fn layer(request: http::Request<Body>, next: Next) -> Response<Body> {
        trace!("Request: {} {}", request.method(), request.uri().path());
        let res = next.run(request).await; // Run the handler, returning the response
        trace!("Response: {}", res.status()); // Read/write the response
        res
    }

    pub fn process_cookie_str(cookie: &str) -> Option<&str> {
        for key_value_pair in cookie.split("; ") {
            let mut itr = key_value_pair.split("=");
            if let Some(key) = itr.next()
                && key == SESSION_KEY
                && let Some(value) = itr.next()
            {
                return Some(value);
            }
        }
        None
    }

    fn get_session_id(headers: &HeaderMap<HeaderValue>) -> Option<&str> {
        if let Some(cookie_value) = headers.get(COOKIE)
            && let Ok(cookie) = cookie_value.to_str()
        {
            return Self::process_cookie_str(cookie);
        }
        None
    }

    pub fn generate_set_cookie(&self) -> String {
        format!("{}={};max-age=31536000;path=/", SESSION_KEY, self.0)
    }

    pub fn generate_remove_cookie() -> String {
        format!("{}=unset;max-age=-1;path=/", SESSION_KEY)
    }

    pub fn get_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Session {
    fn new(user_id: i64) -> (SessionId, Session) {
        (
            SessionId(Uuid::new_v4()),
            Session {
                user_id,
                expire: Local::now().timestamp() + 10000000,
            },
        )
    }

    pub fn expired(&self) -> bool {
        if self.expire != 0 {
            return Local::now().timestamp() > self.expire;
        }
        false
    }

    pub fn find(session_id: &SessionId) -> Option<Session> {
        if let Ok(session_map) = SESSIONS.try_read()
            && let Some(session) = session_map.get(session_id)
            && let Ok(lock) = session.lock()
        {
            return Some(lock.clone());
        }
        None
    }

    pub fn new_and_add(user: RecordUser, password: String) -> Option<SessionId> {
        if validate_password(&password, &user.salt, &user.hash) {
            let (id, session) = Self::new(user.id);
            if let Ok(mut write_guard) = SESSIONS.try_write()
                && write_guard.insert(id.clone(), session.into()).is_none()
            {
                return Some(id);
            }
        }
        None
    }

    pub fn remove(session_uuid_str: &str) -> bool {
        if let Ok(id) = Uuid::parse_str(session_uuid_str)
            && let Ok(mut session_map) = SESSIONS.try_write()
        {
            return session_map.remove(&SessionId(id)).is_some();
        }
        false
    }

    pub fn current() -> Vec<(SessionId, Session)> {
        let mut results = vec![];

        if let Ok(session_map) = SESSIONS.try_read() {
            for (id, session_mutex) in session_map.iter() {
                if let Ok(session) = session_mutex.try_lock() {
                    results.push((id.clone(), session.clone()));
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use axum::http::{HeaderMap, HeaderValue};
    use chrono::Local;
    use uuid::Uuid;

    use crate::{
        backend::auth::{SESSION_KEY, SessionId},
        shared::models::database::model_user::RecordUser,
        static_concat,
    };

    use super::Session;

    fn test_headers(cookie: Option<&str>) -> HeaderMap<HeaderValue> {
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::new();
        headers.insert("method", HeaderValue::from_static("GET"));
        headers.insert("uri", HeaderValue::from_static("/test"));
        if let Some(cookie) = cookie {
            headers.insert("cookie", HeaderValue::from_str(cookie).unwrap());
        }
        headers.insert("connection", HeaderValue::from_static("keep-alive"));
        headers.insert("cache-control", HeaderValue::from_static("no-cache"));
        headers
    }

    #[test]
    fn test_get_session_exists() {
        const SESSION_ID: &str = "b792ca2e-79a1-4ad2-89fb-69defc0ee2d7";
        let headers = test_headers(Some(static_concat!(
            "log-compact=on; log-autoscroll=off; ",
            SESSION_KEY,
            "=",
            SESSION_ID
        )));

        let found_session = SessionId::get_session_id(&headers);
        assert_eq!(SESSION_ID, found_session.unwrap());
    }

    #[test]
    fn test_get_session_empty() {
        let headers = test_headers(Some(static_concat!(
            "log-compact=on; log-autoscroll=off; ",
            SESSION_KEY,
            "="
        )));

        let found_session = SessionId::get_session_id(&headers);
        assert_eq!(Some(""), found_session);
    }

    #[test]
    fn test_get_session_not_exists() {
        let headers = test_headers(None);

        let found_session = SessionId::get_session_id(&headers);
        assert_eq!(None, found_session);
    }

    #[test]
    fn session_expired() {
        let session = Session {
            user_id: 0,
            expire: Local::now().timestamp() - 1000,
        };
        assert!(session.expired())
    }

    #[test]
    fn session_not_expired() {
        let session = Session {
            user_id: 0,
            expire: Local::now().timestamp() + 10000,
        };
        assert!(!session.expired())
    }

    #[test]
    fn session_id_cookie() {
        let uuid = Uuid::new_v4();
        let id_str = uuid.to_string();

        let id = SessionId(uuid);
        let cookie = id.generate_set_cookie();
        assert_eq!(
            format!("{}={};max-age=31536000;path=/", SESSION_KEY, id_str),
            cookie
        );
    }

    #[test]
    fn session_new() {
        let username = "username";
        let password = "raw_password";

        let user = RecordUser::create(username.to_string(), "description".to_string(), password);

        let new_session_id_result = Session::new_and_add(user, password.to_string());
        assert!(
            new_session_id_result.is_some(),
            "Expected new session to be created"
        );
        let new_session_id = new_session_id_result.unwrap();
        assert!(
            Session::find(&new_session_id).is_some(),
            "Expected new session to be added"
        );
        assert!(
            Session::remove(&new_session_id.get_uuid().to_string()),
            "Expected session to be removed"
        );
        assert!(
            Session::find(&new_session_id).is_none(),
            "Expected session to now longer exist"
        );
    }
}
