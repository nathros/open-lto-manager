use std::env;

pub static ENV_PATH_DATA: &'static str = "PATH_DATA"; // TODO uses current working dir, could use /var/lib or ~/.local/share/app_nam
pub static ENV_PATH_DB: &'static str = "PATH_DB";
pub static ENV_PATH_LOG: &'static str = "PATH_LOG";

pub fn get_data_dir() -> String {
    let path = match env::var(ENV_PATH_DATA) {
        Ok(val) => match val.parse::<String>() {
            Ok(val) => val,
            Err(_e) => "data".to_string(),
        },
        Err(_e) => "data".to_string(),
    };
    return path;
}

pub fn get_database_path() -> String {
    let db_path = match env::var(ENV_PATH_DB) {
        Ok(val) => match val.parse::<String>() {
            Ok(val) => val,
            Err(_e) => {
                let mut default = get_data_dir();
                default.push_str("/database");
                return default;
            }
        },
        Err(_e) => {
            let mut default = get_data_dir();
            default.push_str("/database");
            return default;
        }
    };
    db_path
}

pub fn get_database_file() -> String {
    let mut db_file = get_database_path();
    db_file.push_str("/database.db");
    return db_file;
}

pub fn get_logging_path() -> String {
    let log_path = match env::var(ENV_PATH_LOG) {
        Ok(val) => match val.parse::<String>() {
            Ok(val) => val,
            Err(_e) => {
                let mut default = get_data_dir();
                default.push_str("/logs");
                return default;
            }
        },
        Err(_e) => {
            let mut default = get_data_dir();
            default.push_str("/logs");
            return default;
        }
    };
    log_path
}

pub fn get_logging_file() -> String {
    let log_file_path = get_logging_path();
    let mut log_file = log_file_path.clone();
    log_file.push_str("/main.log");
    return log_file;
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::backend::env::{
        get_data_dir, get_database_path, get_logging_path, ENV_PATH_DATA, ENV_PATH_DB, ENV_PATH_LOG,
    };

    #[test]
    fn composite_env() {
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
    }
}
