use crate::shared::models::tape_drive::TapeDrive;
use std::{fs::read_dir, path::MAIN_SEPARATOR};

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
