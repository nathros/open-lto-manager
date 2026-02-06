use dioxus::prelude::*;

use crate::shared::models::file_view::FileView;

#[get("/api/fv/working_dir")]
pub async fn fv_working_dir() -> Result<String> {
    let working_dir = std::env::current_dir().unwrap_or([r""].iter().collect());
    std::thread::sleep(std::time::Duration::from_millis(1000));
    Ok(working_dir
        .into_os_string()
        .into_string()
        .unwrap_or("".to_string()))
}

#[post("/api/fv/explore")]
pub async fn fv_files_in_dir(path: String) -> Result<Result<Vec<FileView>, String>> {
    use std::fs::read_dir;
    use std::os::unix::fs::MetadataExt;
    use std::{thread, time::Duration};

    let mut result = vec![];

    thread::sleep(Duration::from_millis(1));

    let read_dir = match read_dir(&path) {
        Ok(dir) => dir,
        Err(e) => {
            // return Ok(Err(CapturedError::msg(format!("Cannot find dir {}", path))));
            return Ok(Err(format!("Error: {}", e)));
        }
    };

    for path in read_dir.flatten() {
        if let Ok(metadata) = path.metadata() {
            result.push(FileView {
                is_dir: metadata.is_dir(),
                file_name: path
                    .file_name()
                    .into_string()
                    .unwrap_or("Failed to read filename".to_string()),
                size: metadata.size(),
            });
        }
    }
    Ok(Ok(result))
}
