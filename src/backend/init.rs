use crate::backend::database::db::create_database;
use crate::{backend::logging::LOG_LAYERS, shared::models::app_state::AppState};
use std::sync::LazyLock;
use std::vec;
use tracing::{error, info};

pub static APP_STATE: LazyLock<AppState> = LazyLock::new(init_backend);

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

    let (ltfs_installed, ltfs_error) = match std::process::Command::new("ltfs")
        .stdout(std::process::Stdio::null()) // Hide output from console
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(_) => {
            info!("Init [ltfs] found");
            (true, None)
        }
        Err(e) => {
            if std::io::ErrorKind::NotFound != e.kind() {
                error!("Init [ltfs] error: {}", e);
                (false, Some(format!("{}", e)))
            } else {
                error!("Init [ltfs] not found");
                (false, None)
            }
        }
    };

    let mt_installed = match std::process::Command::new("mt")
        .stdout(std::process::Stdio::null()) // Hide output from console
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(_) => {
            info!("Init [mt] found");
            true
        }
        Err(e) => {
            if std::io::ErrorKind::NotFound != e.kind() {
                error!("Init [mt] error: {}", e);
            } else {
                error!("Init [mt] not found");
            }
            false
        }
    };

    AppState {
        user_name,
        user_name_error,
        part_tape_group,
        ltfs_installed,
        ltfs_error,
        mt_installed,
        platform: whoami::platform().to_string(),
        cpu_arch: whoami::cpu_arch().to_string(),
        distro: whoami::distro().unwrap_or("unknown".to_string()),
        critical_error: log_error || database_result.is_err(),
        error_list,
    }
}
