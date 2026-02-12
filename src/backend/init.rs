use crate::backend::database::db::create_database;
use crate::{backend::logging::LOG_LAYERS, shared::models::app_state::AppState};
use std::sync::LazyLock;
use std::vec;
use tracing::{error, info};

pub static APP_STATE: LazyLock<AppState> = LazyLock::new(init_backend);

pub fn init_backend() -> AppState {
    let mut error_list = vec![];

    let log_error = match LOG_LAYERS.as_ref() {
        Ok(_log_file_layer) => false,
        Err(error) => {
            error!("Logging init error: {}", error);
            error_list.push(error.clone());
            true
        }
    };
    info!("App init");

    let database_result = create_database();
    if let Some(error) = database_result.as_ref().err() {
        error!("Database init error: {}", error);
        error_list.push(error.clone());
    }

    AppState {
        user_name: whoami::username().unwrap_or_else(|_| "unknown".to_string()),
        platform: whoami::platform().to_string(),
        cpu_arch: whoami::cpu_arch().to_string(),
        critical_error: log_error || database_result.is_err(),
        error_list,
    }
}
