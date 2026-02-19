use dioxus::prelude::*;

use crate::shared::models::file_view::FileView;

#[get("/api/fv/working_dir")]
pub async fn fv_working_dir() -> Result<String> {
    #[cfg(feature = "slow_server")]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    let working_dir = std::env::current_dir().unwrap_or([r""].iter().collect());
    Ok(working_dir
        .into_os_string()
        .into_string()
        .unwrap_or("".to_string()))
}

#[post("/api/fv/explore")]
pub async fn fv_files_in_dir(
    path: String,
    expanded: bool,
    nest_index: usize,
) -> Result<Result<Vec<FileView>, String>> {
    use std::fs::read_dir;
    use std::os::unix::fs::MetadataExt;
    use std::path::MAIN_SEPARATOR;

    #[cfg(feature = "slow_server")]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    info!("fetch2 {}", path);

    if path.is_empty() {
        return Ok(Ok(vec![]));
    }

    let read_dir = match read_dir(&path) {
        Ok(dir) => dir,
        Err(e) => {
            return Ok(Err(format!("{}", e)));
        }
    };

    let mut results = vec![];

    for current_path in read_dir.flatten() {
        let name = current_path
            .file_name()
            .into_string()
            .unwrap_or("Failed to read filename".to_string());

        if let Ok(metadata) = current_path.metadata() {
            results.push(FileView {
                is_dir: metadata.is_dir(),
                path: format!("{}{}{}", path.clone(), MAIN_SEPARATOR, name),
                name,
                size: if metadata.is_dir() {
                    0
                } else {
                    metadata.size()
                },
                expanded,
                nest: nest_index,
                hidden: false,
                selected: false,
            });
        }
    }

    Ok(Ok(results))
}
