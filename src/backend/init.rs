use std::{sync::LazyLock, vec};
use tokio::runtime::Runtime;
use tracing::{error, info};

use crate::{
    backend::{database::db::create_database, logging::LOG_LAYERS},
    shared::models::app_state::AppState,
};

use super::system::shell::shell_command_blocking::shell_command_output_blocking;

pub static APP_STATE: LazyLock<AppState> = LazyLock::new(init_backend);

// Check command exists
fn check_command(command: &str) -> (bool, Option<String>) {
    match std::process::Command::new(command)
        .stdout(std::process::Stdio::null()) // Hide output from console
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(mut child) => {
            // Need to wait otherwise child will be marked as defunct
            if let Some(e) = child.wait().err() {
                error!("Init [{}] found but has error: {}", command, e);
                (true, Some(format!("{}", e)))
            } else {
                info!("Init [{}] found", command);
                (true, None)
            }
        }
        Err(e) => {
            if std::io::ErrorKind::NotFound != e.kind() {
                error!("Init [{}] error: {}", command, e);
                (false, Some(format!("{}", e)))
            } else {
                error!("Init [{}] not found", command);
                (false, None)
            }
        }
    }
}

fn get_latest_ltfs_version() -> Option<String> {
    match Runtime::new() {
        Ok(rt) => rt.block_on(async {
            // Start:   https://github.com/LinearTapeFileSystem/ltfs/releases/latest
            // Will redirect to latest tag:
            // Example: https://github.com/LinearTapeFileSystem/ltfs/releases/tag/v2.4.8.2-10520
            // Get substring between pipes |                                     >|-------|<
            match reqwest::get("https://github.com/LinearTapeFileSystem/ltfs/releases/latest").await // FIXME check timeout
            {
                Ok(response) => {
                    let url = response.url().to_string(); // Redirected URL
                    let find = "tag/";
                    match url.find(find) {
                        Some(mut start_index) => {
                            let end_index = url.find("-").unwrap_or(url.len());
                            start_index += find.len() + 1; // Skip "tag/"
                            if start_index < end_index {
                                let result = &url[start_index..end_index];
                                info!("Init [ltfs-latest] : {}", result);
                                Some(result.to_string())
                            } else {
                                error!("Init [ltfs-latest] failed to find tag: {}", url);
                                None
                            }
                        },
                        None => {
                            error!("Init [ltfs-latest] failed to find tag: {}", url);
                            None
                        },
                    }
                },
                Err(e) => {
                    error!("Init [ltfs-latest] failure: {}", e);
                    None
                }
            }
        }),
        Err(e) => {
            error!("Init [ltfs-latest] rt failure: {}", e);
            None
        }
    }
}

fn get_current_ltfs_version() -> Option<String> {
    match shell_command_output_blocking("ltfs", vec!["--version"]) {
        Ok((_stdout, stderr)) => {
            if let Some(ver) = stderr.first() {
                let find = "version ";
                return match ver.find("version ") {
                    Some(mut start_index) => {
                        let end_index = ver.find(" (").unwrap_or(ver.len());
                        start_index += find.len(); // Skip "version "
                        if start_index < end_index {
                            let result = &ver[start_index..end_index];
                            info!("Init [ltfs-current]: {}", result);
                            return Some(result.to_string());
                        }
                        error!("Init [ltfs-current]: {}", ver);
                        None
                    }
                    None => {
                        error!("Init [ltfs-current]: {}", ver);
                        None
                    }
                };
            }
            error!("Init [ltfs-current]: Failed to find ");
            None
        }
        Err(e) => {
            error!("Init [ltfs-current]: {}", e);
            None
        }
    }
}

fn init_backend() -> AppState {
    let mut error_list = vec![];

    let log_error = match LOG_LAYERS.as_ref() {
        Ok(_log_file_layer) => false,
        Err(error) => {
            error!("Init [logging] error: {}", error);
            error_list.push(error.clone());
            true
        }
    };
    info!("App init");

    let database_result = create_database();
    if let Some(error) = database_result.as_ref().err() {
        error!("Init [database] error: {}", error);
        error_list.push(error.clone());
    } else {
        info!("Init [database] success");
    }

    let (user_name, user_name_error) = match whoami::username() {
        Ok(name) => (name, None),
        Err(e) => ("unknown".to_string(), Some(format!("{}", e))),
    };

    let part_tape_group = match uzers::get_user_by_name(&user_name) {
        Some(current_user) => {
            if let Some(groups) = current_user.groups() {
                groups.iter().any(|g| g.name() == "tape")
            } else {
                false
            }
        }
        None => false,
    };
    if part_tape_group {
        info!("Init [groups] user '{}' found in 'tape' group", user_name);
    } else {
        error!(
            "Init [groups] user '{}', not found in 'tape' group",
            user_name
        );
    }

    let (ltfs_installed, ltfs_error) = check_command("ltfs");
    let (mt_installed, mt_error) = check_command("mt");
    let ltfs_version = get_current_ltfs_version();
    let ltfs_version_latest = get_latest_ltfs_version();
    let latest_is_newer = if let Some(current) = ltfs_version.as_ref()
        && let Some(latest) = ltfs_version_latest.as_ref()
    {
        latest > current
    } else {
        false
    };

    AppState {
        user_name,
        user_name_error,
        part_tape_group,
        ltfs_installed,
        ltfs_version,
        ltfs_version_latest,
        latest_is_newer,
        ltfs_error,
        mt_installed,
        mt_error,
        platform: whoami::platform().to_string(),
        cpu_arch: whoami::cpu_arch().to_string(),
        distro: whoami::distro().unwrap_or("unknown".to_string()),
        critical_error: log_error || database_result.is_err(),
        error_list,
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::init::get_latest_ltfs_version;

    #[test]
    fn latest_ltfs() {
        let result = get_latest_ltfs_version(); // Expect something like: "2.4.8.2"
        assert!(result.is_some(), "Failed to get LTFS version");
        let mut prev_was_dot = false;
        let mut prev_was_num = false;
        let mut first_char = true;
        for char in result.unwrap().chars() {
            match char {
                '0'..='9' => {
                    prev_was_dot = false;
                    prev_was_num = true;
                    first_char = false;
                }
                '.' => {
                    if prev_was_dot {
                        unreachable!("Found consecutive dots");
                    }
                    if first_char {
                        unreachable!("First char is dot");
                    }
                    prev_was_dot = true;
                    prev_was_num = false;
                }
                _ => {
                    unreachable!("Invalid char")
                }
            }
        }
        assert!(prev_was_num, "Should end with number");
    }
}
