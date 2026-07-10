use std::{fs, sync::LazyLock, vec};
use tokio::runtime::Runtime;
use tracing::{error, info, warn};

use crate::{
    backend::{database::db::create_database, logging::LOG_LAYERS},
    shared::models::app_state::AppState,
};

use super::system::shell::shell_command_blocking::shell_command_output_blocking;

pub static APP_STATE: LazyLock<AppState> = LazyLock::new(init_backend);
const GROUP_ANY: &str = "tape";
const GROUP_ARCH: &str = "storage";

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

fn get_os_release_id_like(path: &str) -> Option<String> {
    // Look in /etc/os_release for key value pair
    // ID_LIKE={value}
    if let Ok(file_as_str) = fs::read_to_string(path) {
        for key_value in file_as_str.split("\n") {
            let mut itr = key_value.split("=");
            if let Some(key) = itr.next()
                && key == "ID_LIKE"
                && let Some(value) = itr.next()
            {
                info!("Init [os] {} found ID_LIKE={}", path, value);
                return Some(value.to_string());
            }
        }
    }
    warn!("Init [os] {} failed to find ID_LIKE", path);
    None
}

fn get_needed_system_group(os_release_path: &str, fallback_name: &String) -> String {
    // Most distros use the tape group, Arch(+derivatives) uses storage
    // https://wiki.archlinux.org/title/Users_and_groups
    let os_release_id_like = get_os_release_id_like(os_release_path);
    let os_details = os_release_id_like.as_ref().unwrap_or(fallback_name);
    match os_details.to_lowercase().find("arch").is_some() {
        true => GROUP_ARCH.to_string(),
        false => GROUP_ANY.to_string(),
    }
}

fn init_backend() -> AppState {
    let mut error_list = vec![];
    let mut pass_count = 0;
    let mut warn_count = 0;
    let mut err_count = 0;

    let log_error = match LOG_LAYERS.as_ref() {
        Ok(_log_file_layer) => false,
        Err(error) => {
            error!("Init [logging] error: {}", error);
            error_list.push(error.clone());
            err_count += 1;
            true
        }
    };
    info!("App init");

    let database_result = create_database();
    if let Some(error) = database_result.as_ref().err() {
        error!("Init [database] error: {}", error);
        error_list.push(error.clone());
        err_count += 1;
    } else {
        info!("Init [database] success");
    }

    let distro = whoami::distro().unwrap_or("unknown".to_string());
    let group = get_needed_system_group("/etc/os-release", &distro);
    info!("Init [os] OS: {}", distro);

    let user_name = match whoami::username() {
        Ok(name) => Some(name),
        Err(e) => {
            error_list.push(format!("{}", e));
            err_count += 1;
            None
        }
    };

    let user_part_of_group = match user_name.as_deref() {
        Some(found_user) => match uzers::get_user_by_name(found_user) {
            Some(current_user) => {
                if let Some(groups) = current_user.groups() {
                    groups.iter().any(|g| *g.name() == *group)
                } else {
                    false
                }
            }
            None => false,
        },
        None => false,
    };

    if let Some(found_user) = user_name.as_deref() {
        if user_part_of_group {
            info!("Init [groups] user '{}' found in 'tape' group", found_user);
        } else {
            warn!(
                "Init [groups] user '{}', not found in 'tape' group",
                found_user
            );
            warn_count += 1;
        }
    } else {
        error!("Init [user] Unable to find current user");
        error_list.push("Unable to find current user".to_string());
        err_count += 1;
    }

    let (ltfs_installed, ltfs_error) = check_command("ltfs");
    if ltfs_installed {
        pass_count += 1;
    } else {
        warn_count += 1;
    }
    if ltfs_error.is_some() {
        warn_count += 1;
    }

    let (mt_installed, mt_error) = check_command("mt");
    let ltfs_version = get_current_ltfs_version();
    let ltfs_version_latest = get_latest_ltfs_version();
    let ltfs_latest_is_newer = if let Some(current) = ltfs_version.as_ref()
        && let Some(latest) = ltfs_version_latest.as_ref()
    {
        latest > current
    } else {
        false
    };
    if ltfs_latest_is_newer {
        warn_count += 1;
    }

    AppState {
        user_name,
        group,
        user_part_of_group,
        ltfs_installed,
        ltfs_version,
        ltfs_version_latest,
        ltfs_latest_is_newer,
        ltfs_error,
        mt_installed,
        mt_error,
        platform: whoami::platform().to_string(),
        cpu_arch: whoami::cpu_arch().to_string(),
        distro,
        critical_error: log_error || database_result.is_err(),
        error_list,
        pass_count,
        warn_count,
        err_count,
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempdir::TempDir;

    use crate::backend::init::{
        GROUP_ANY, GROUP_ARCH, get_latest_ltfs_version, get_needed_system_group,
    };

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

    #[test]
    fn check_platform_group() {
        let catchy_os_release = r#"NAME="CachyOS Linux"
PRETTY_NAME="CachyOS"
ID=cachyos
ID_LIKE=arch
BUILD_ID=rolling
VERSION_ID=20260705.0.552420
ANSI_COLOR="38;2;23;147;209"
HOME_URL="https://cachyos.org/"
DOCUMENTATION_URL="https://wiki.cachyos.org/"
SUPPORT_URL="https://discuss.cachyos.org/"
BUG_REPORT_URL="https://github.com/cachyos"
PRIVACY_POLICY_URL="https://terms.archlinux.org/docs/privacy-policy/"
LOGO=cachyos"#;

        let arch_os_release = r#"NAME="Arch Linux"
PRETTY_NAME="Arch Linux"
ID=arch
BUILD_ID=rolling
VERSION_ID=20260222.0.493200
ANSI_COLOR="38;2;23;147;209"
HOME_URL="https://archlinux.org/"
DOCUMENTATION_URL="https://wiki.archlinux.org/"
SUPPORT_URL="https://bbs.archlinux.org/"
BUG_REPORT_URL="https://gitlab.archlinux.org/groups/archlinux/-/issues"
PRIVACY_POLICY_URL="https://terms.archlinux.org/docs/privacy-policy/"
LOGO=archlinux-logo"#;

        let ubuntu_os_release = r#"PRETTY_NAME="Ubuntu 26.04 LTS"
NAME="Ubuntu"
VERSION_ID="26.04"
VERSION="26.04 LTS (Resolute Raccoon)"
VERSION_CODENAME=resolute
ID=ubuntu
ID_LIKE=debian
HOME_URL="https://www.ubuntu.com/"
SUPPORT_URL="https://help.ubuntu.com/"
BUG_REPORT_URL="https://bugs.launchpad.net/ubuntu/"
PRIVACY_POLICY_URL="https://www.ubuntu.com/legal/terms-and-policies/privacy-policy"
UBUNTU_CODENAME=resolute
LOGO=ubuntu-logo"#;

        let test_data = [
            (catchy_os_release, "CachyOS Linux".to_string(), GROUP_ARCH),
            (arch_os_release, "Arch Linux".to_string(), GROUP_ARCH),
            (ubuntu_os_release, "Ubuntu 26.04 LTS".to_string(), GROUP_ANY),
        ];

        let tmp_dir = TempDir::new("os_release").unwrap();
        for (index, (os_release_content, os_name, expected_result)) in test_data.iter().enumerate()
        {
            let tmp_file_path = tmp_dir.path().join(format!("{}", index));
            // Write temporary file 'os_release'
            fs::write(&tmp_file_path, os_release_content).unwrap();
            let tmp_file_path_str = tmp_file_path.into_os_string().into_string().unwrap();

            let result = get_needed_system_group(tmp_file_path_str.as_str(), os_name);
            assert_eq!(result.as_str(), *expected_result);
        }
        tmp_dir.close().unwrap(); // Clear tmp dir
    }
}
