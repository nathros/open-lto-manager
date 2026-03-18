use std::{sync::LazyLock, vec};
use tracing::{error, info};

use crate::{
    backend::{database::db::create_database, logging::LOG_LAYERS},
    shared::models::app_state::AppState,
};

pub static APP_STATE: LazyLock<AppState> = LazyLock::new(init_backend);

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

    AppState {
        user_name,
        user_name_error,
        part_tape_group,
        ltfs_installed,
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
