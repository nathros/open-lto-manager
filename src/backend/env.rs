static ENV_PATH_DATA: &str = "PATH_DATA"; // TODO uses current working dir, could use /var/lib or ~/.local/share/app_nam
static ENV_PATH_DB: &str = "PATH_DB";
static ENV_PATH_LOG: &str = "PATH_LOG";
static ENV_CONSOLE_LOG: &str = "CONSOLE_LOG";
static ENV_DB_PEPPER: &str = "DB_PEPPER";

pub fn get_data_dir() -> String {
    std::env::var(ENV_PATH_DATA)
        .ok()
        .and_then(|p| p.parse::<String>().ok())
        .unwrap_or("data".to_string())
}

pub fn get_database_path() -> String {
    std::env::var(ENV_PATH_DB)
        .ok()
        .and_then(|p| p.parse::<String>().ok())
        .unwrap_or(format!("{}/database", get_data_dir()))
}

pub fn get_database_file(path: &String) -> String {
    format!("{}/database.db", path)
}

pub fn get_logging_path() -> String {
    std::env::var(ENV_PATH_LOG)
        .ok()
        .and_then(|p| p.parse::<String>().ok())
        .unwrap_or(format!("{}/logs", get_data_dir()))
}

pub fn get_logging_file() -> String {
    format!("{}/main.log", get_logging_path())
}

pub fn get_console_log_enabled() -> bool {
    std::env::var(ENV_CONSOLE_LOG)
        .ok()
        .and_then(|p| p.parse::<String>().ok().map(|f| f == "ON"))
        .unwrap_or(false)
}

pub fn get_pepper() -> Option<String> {
    std::env::var(ENV_DB_PEPPER).ok()
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::backend::env::{
        ENV_CONSOLE_LOG, ENV_DB_PEPPER, ENV_PATH_DATA, ENV_PATH_DB, ENV_PATH_LOG,
        get_console_log_enabled, get_data_dir, get_database_path, get_logging_path, get_pepper,
    };

    #[test]
    fn composite_env() {
        unsafe {
            // Unsafe due to set_var and remove_var, tests are typically run in parallel threads
            // So all env tests that use set_var and remove_var can only be in this test

            // Data path
            assert_eq!(get_data_dir(), "data".to_string());
            // Note: this is program wide, so other tests that use env will be affected
            env::set_var(ENV_PATH_DATA, "new_value");
            assert_eq!(get_data_dir(), "new_value".to_string());
            env::remove_var(ENV_PATH_DATA); // Reset

            // Database path
            assert_eq!(get_database_path(), "data/database".to_string());
            // If only data path is set
            env::set_var(ENV_PATH_DATA, "new_data_path");
            assert_eq!(get_database_path(), "new_data_path/database".to_string());
            // If database path is set
            env::set_var(ENV_PATH_DB, "test_data_path");
            assert_eq!(get_database_path(), "test_data_path".to_string());
            env::remove_var(ENV_PATH_DATA); // Reset
            env::remove_var(ENV_PATH_DB); // Reset

            // Logging path
            assert_eq!(get_logging_path(), "data/logs".to_string());
            // If only data path is set
            env::set_var(ENV_PATH_DATA, "new_data_path");
            assert_eq!(get_logging_path(), "new_data_path/logs".to_string());
            // If database path is set
            env::set_var(ENV_PATH_LOG, "test_data_path");
            assert_eq!(get_logging_path(), "test_data_path".to_string());
            env::remove_var(ENV_PATH_DATA); // Reset
            env::remove_var(ENV_PATH_LOG); // Reset

            // Console log enabled
            assert!(!get_console_log_enabled());
            env::set_var(ENV_CONSOLE_LOG, "OFF");
            assert!(!get_console_log_enabled());
            env::set_var(ENV_CONSOLE_LOG, "INVALID");
            assert!(!get_console_log_enabled());
            env::set_var(ENV_CONSOLE_LOG, "ON");
            assert!(get_console_log_enabled());
            env::remove_var(ENV_CONSOLE_LOG); // Reset

            // Database pepper
            assert!(get_pepper().is_none());
            env::set_var(ENV_DB_PEPPER, "1234");
            assert_eq!(get_pepper().unwrap(), "1234".to_string());
            env::remove_var(ENV_DB_PEPPER); // Reset
        }
    }
}
