use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AppState {
    pub critical_error: bool,
    pub error_list: Vec<String>,
}

#[cfg(feature = "server")]
pub static APP_STATE: LazyLock<AppState> = LazyLock::new(|| init_backend());

#[cfg(feature = "server")]
pub fn init_backend() -> AppState {
    use std::vec;

    use crate::backend::{database::db::create_database, logging::setup_logging};
    use tracing::error;

    let mut error_list = vec![];

    let logging_result = setup_logging();
    if let Some(error) = logging_result.as_ref().err() {
        error!("Logging init error: {}", error);
        error_list.push(error.clone());
    }

    let database_result = create_database();
    if let Some(error) = database_result.as_ref().err() {
        error!("Database init error: {}", error);
        error_list.push(error.clone());
    }

    AppState {
        critical_error: logging_result.is_err() || database_result.is_err(),
        error_list,
    }
}
