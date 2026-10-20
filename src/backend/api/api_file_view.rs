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
        let name: String = current_path
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

#[post("/api/fv/virtual/explore", user_id: crate::backend::auth::UserIdSession)]
pub async fn fv_virtual_files_in_dir(path: String) -> Result<FileView> {
    use crate::backend::database::{db::DB, tables::file::table_file::TableFile};

    DB.with(|db: &std::sync::LazyLock<rusqlite::Connection>| {
        match TableFile::<FileView>::get_in_virtual_dir(db, user_id.0, path) {
            Ok(records) => Ok(records),
            Err(e) => Err(e)?,
        }
    })
}

#[post("/api/fv/physical/explore")]
pub async fn fv_physical_files_in_dir(mut path: String) -> Result<FileView> {
    use std::{
        fs::read_dir, os::unix::fs::MetadataExt, path::MAIN_SEPARATOR, path::Path, time::SystemTime,
    };

    use chrono::Local;

    use crate::{backend::env::get_working_dir, shared::models::file_view::FileViewInfo};

    if path.is_empty() {
        path = get_working_dir();
        path.push(MAIN_SEPARATOR);
    } else {
        if !path.ends_with(MAIN_SEPARATOR) {
            path.push(MAIN_SEPARATOR);
        }
    }

    let current_dir = match read_dir(&path) {
        Ok(dir) => dir,
        Err(_e) => {
            return Ok(FileView::default()); // Did not find dir return empty, TODO could return error reason
        }
    };

    let mut children = current_dir
        .flatten()
        .filter(|fe| fe.metadata().is_ok())
        .map(|fe: std::fs::DirEntry| {
            if let name = fe
                .file_name()
                .into_string()
                .unwrap_or("Failed to read filename".to_string())
                && let Ok(metadata) = fe.metadata()
            {
                FileViewInfo {
                    id: 0,
                    is_selected: false,
                    is_dir: metadata.is_dir(),
                    icon: FileViewInfo::get_icon_default(metadata.is_dir(), &name),
                    file_name: name,
                    file_size: metadata.size() as i64,
                    file_size_pretty: FileViewInfo::pretty_size(metadata.size() as i64),
                    created: metadata.created().unwrap_or(SystemTime::now()).into(),
                    modified: metadata.modified().unwrap_or(SystemTime::now()).into(),
                }
            } else {
                unreachable!("Filter not correctly applied")
            }
        })
        .collect::<Vec<FileViewInfo>>();

    FileViewInfo::sort(&mut children);

    let current_path = Path::new(&path);

    let result = FileView {
        current_dir: Some(FileViewInfo {
            id: 0,
            is_selected: false,
            is_dir: true,
            file_name: current_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_size: 0,
            file_size_pretty: "".to_string(),
            created: Local::now(),
            modified: Local::now(),
            icon: FileViewInfo::default_dir_icon(),
        }),
        children,
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::backend::api::api_file_view::{fv_physical_files_in_dir, fv_working_dir};

    #[tokio::test]
    async fn fetch_physical_working_dir() {
        let test_path = fv_working_dir().await.unwrap();
        let test_files = fv_physical_files_in_dir(test_path).await.unwrap();

        assert!(test_files.current_dir.is_some(), "Did not find directory");
        assert!(
            !test_files.children.is_empty(),
            "Expected some children files"
        );
    }
}
