use std::{fs::read_dir, path::MAIN_SEPARATOR, process::Stdio};
use tokio::{process::Command, sync::broadcast::Sender};
use tracing::{error, info};

use crate::{
    backend::system::shell::{
        common::shell_output_default,
        shell_command::{
            SHELL_SERVICES, TaskCompleteFn, TaskGuard, close_shell_service, create_shell_service,
        },
    },
    shared::models::tape_drive::TapeDrive,
};

pub fn get_current_tape_devices() -> Result<Vec<TapeDrive>, String> {
    check_dir("/dev")
}

fn check_dir(path: &str) -> Result<Vec<TapeDrive>, String> {
    let mut devices = vec![];

    let read_dir = match read_dir(path) {
        Ok(dir) => dir,
        Err(e) => {
            return Err(format!("{}", e));
        }
    };

    for current_path in read_dir.flatten() {
        let name = current_path
            .file_name()
            .into_string()
            .unwrap_or("Failed to read filename".to_string());
        if name.starts_with("nst") {
            devices.push(TapeDrive {
                dev: format!("{}{}{}", path, MAIN_SEPARATOR, name),
                manufacturer: "[manufacturer todo!()]".to_string(),
            });
        }
    }

    Ok(devices)
}

pub fn check_devices(id: i64) -> Result<Sender<String>, String> {
    create_shell_service(
        &SHELL_SERVICES,
        id,
        Box::new(check_devices_shell),
        Box::new(move || close_shell_service(&SHELL_SERVICES, id)),
    )
}

fn check_devices_shell(sender: Sender<String>, on_complete: TaskCompleteFn) {
    let guard = TaskGuard {
        complete_fn: on_complete, // This will call on_complete on drop
    };

    let args = &["./scripts/test.sh"];
    let cmd = "bash";

    info!("Run background shell command: {:?} {:?}", cmd, args);

    match Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(mut cmd) => {
            if let Some(stdout) = cmd.stdout.take()
                && let Some(stderr) = cmd.stderr.take()
            {
                tokio::spawn(async move {
                    let _guard_keep_alive = guard; // Keep guard alive until all resolved
                    shell_output_default(sender, stdout, stderr).await;
                }); // Guard dropped here
            } else {
                error!(
                    "Failed to get stdout/err, stopping command: {:?} {:?}",
                    cmd, args
                );
                let _ = cmd.start_kill();
            }
        }
        Err(e) => {
            error!("Failed to start command: {}", e);
            if let Err(e) = sender.send(format!("{}", e)) {
                error!("Failed propagate error to subscriber: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use tempdir::TempDir;

    use crate::backend::system::devices::check_dir;

    #[test]
    fn find_devices() {
        let tmp_dir = TempDir::new("test-find-tape-devices").unwrap();

        let mut tmp_devices = vec![];

        let index_ranges = (1..5).chain(9..10).collect::<Vec<_>>(); // With Gap
        let outside_ranges = (7..8).chain(12..14).collect::<Vec<_>>(); // With Gap

        for i in index_ranges.iter() {
            let file_path = tmp_dir.path().join(format!("nst{}", i));
            let tmp_file = File::create(file_path).unwrap();
            tmp_devices.push(tmp_file);
        }
        for i in index_ranges.iter() {
            let file_path = tmp_dir.path().join(format!("sda{}", i));
            let tmp_file = File::create(file_path).unwrap();
            tmp_devices.push(tmp_file);
        }

        let found_devices = check_dir(tmp_dir.path().as_os_str().to_str().unwrap()).unwrap();
        // Should find these devices
        for i in index_ranges {
            assert!(
                found_devices
                    .iter()
                    .any(|device| device.dev.ends_with(format!("nst{}", i).as_str()))
            );
        }

        // Should not find these
        for i in outside_ranges {
            assert!(
                !found_devices
                    .iter()
                    .any(|device| device.dev.ends_with(format!("nst{}", i).as_str()))
            );
        }

        tmp_dir.close().unwrap();
    }
}
