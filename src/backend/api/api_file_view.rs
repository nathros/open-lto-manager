use dioxus::prelude::*;

use crate::shared::models::{file_view::FileView, file_view_legacy::FileViewLegacy};

#[get("/api/fv/working_dir")]
pub async fn fv_working_dir() -> Result<String> {
    use crate::backend::env::get_working_dir;

    Ok(get_working_dir())
}

#[post("/api/fv/explore")]
pub async fn fv_files_in_dir(
    path: String,
    expanded: bool,
    nest_index: usize,
) -> Result<Result<Vec<FileViewLegacy>, String>> {
    use std::fs::read_dir;
    use std::os::unix::fs::MetadataExt;
    use std::path::MAIN_SEPARATOR;

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
            results.push(FileViewLegacy {
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

#[post("/api/fv/virtual/explore")]
pub async fn fv_virtual_files_in_dir(path: String) -> Result<FileView> {
    use crate::backend::database::{db::DB, tables::file::table_file::TableFile};

    DB.with(|db: &std::sync::LazyLock<rusqlite::Connection>| {
        match TableFile::<FileView>::get_in_virtual_dir(db, path) {
            Ok(records) => Ok(records),
            Err(e) => Err(e)?,
        }
    })
}
