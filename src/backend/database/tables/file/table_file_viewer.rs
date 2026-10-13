use std::path::Path;

use rusqlite::{Connection, params};
use tracing::info;

use crate::{
    backend::database::tables::{file::table_file::TableFile, table::RecordFill},
    shared::models::{
        database::file::model_file::RecordFile,
        file_view::{FileView, FileViewInfo},
    },
};

impl RecordFill<FileViewInfo> for TableFile<FileViewInfo> {
    fn fill(row: &rusqlite::Row<'_>, offset: usize) -> Result<FileViewInfo, rusqlite::Error> {
        let file_name_db: String = row.get(offset + 1)?;
        let is_dir = FileViewInfo::is_dir(&file_name_db);
        Ok(FileViewInfo {
            id: row.get(offset)?,
            icon: if is_dir {
                let ico: String = row.get(offset + 6)?;
                if ico.is_empty() {
                    "#folder".to_string() // Default for dir
                } else {
                    ico
                }
            } else {
                [
                    "#", // Anchor for sprite
                    &Path::new(&file_name_db)
                        .extension()
                        .unwrap_or_default()
                        .to_string_lossy(),
                ]
                .concat()
            },
            is_dir,
            is_selected: false,
            file_name: FileViewInfo::trim_name(file_name_db),
            file_path: row.get(offset + 2)?,
            file_size: row.get(offset + 3)?,
            created: row.get(offset + 4)?,
            modified: row.get(offset + 5)?,
        })
    }
}

impl TableFile<FileView> {
    pub fn get_in_virtual_dir(db: &Connection, dir: String) -> Result<FileView, rusqlite::Error> {
        let (path, name, directory) = TableFile::<RecordFile>::split_dir(dir.clone());
        info!(
            "dir: {}, path: {}, name: {}, directory: {}",
            dir, path, name, directory
        );
        let dir_result: Result<FileViewInfo, rusqlite::Error> = db
            .prepare(
                "SELECT
                        id,
                        file_name_virt,
                        file_path_virt,
                        file_size,
                        created,
                        modified,
                        hash,
                        icon
                    FROM file
                    WHERE
                        file_path_virt = ?1 AND file_name_virt = ?2",
            )?
            .query_one(params![path, name], |row| TableFile::fill(row, 0));

        match dir_result {
            Ok(dir) => {
                let children_result = db
                    .prepare(
                        "SELECT
                                id,
                                file_name_virt,
                                file_path_virt,
                                file_size,
                                created,
                                modified,
                                hash,
                                icon
                            FROM file
                            WHERE
                                file_path_virt LIKE ?1 AND NOT id = ?2",
                    )?
                    .query_map(params![directory, dir.id], |row| TableFile::fill(row, 0))?
                    .collect::<Result<Vec<FileViewInfo>, rusqlite::Error>>()?;

                Ok(FileView {
                    current_dir: Some(dir),
                    children: children_result,
                })
            }
            Err(e) => match e {
                rusqlite::Error::QueryReturnedNoRows => Ok(FileView {
                    current_dir: None,
                    children: vec![],
                }),
                _ => Err(e),
            },
        }
    }
}
