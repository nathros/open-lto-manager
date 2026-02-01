use std::sync::LazyLock;

use tracing::{info, level_filters::LevelFilter};

use crate::{
    backend::logging::{change_file_logger_level, FILE_LOG},
    shared::models::app_state::AppState,
};

pub static APP_STATE: LazyLock<AppState> = LazyLock::new(|| init_backend());

pub fn init_backend() -> AppState {
    use crate::backend::database::db::create_database;
    use std::vec;
    use tracing::error;

    let mut error_list = vec![];

    let log_error = match FILE_LOG.as_ref() {
        Ok(_log_file_layer) => {
            change_file_logger_level(LevelFilter::INFO);
            info!("Set file logger level to: INFO");
            false
        }
        Err(error) => {
            error!("Logging init error: {}", error);
            error_list.push(error.clone());
            true
        }
    };

    let database_result = create_database();
    if let Some(error) = database_result.as_ref().err() {
        error!("Database init error: {}", error);
        error_list.push(error.clone());
    }

    AppState {
        critical_error: log_error || database_result.is_err(),
        error_list,
    }
}
