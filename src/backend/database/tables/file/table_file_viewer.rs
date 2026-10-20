use rusqlite::{Connection, params};

use crate::{
    backend::database::tables::{file::table_file::TableFile, table::RecordFill},
    shared::models::file_view::{FileView, FileViewInfo},
};

impl RecordFill<FileViewInfo> for TableFile<FileViewInfo> {
    fn fill(row: &rusqlite::Row<'_>, offset: usize) -> Result<FileViewInfo, rusqlite::Error> {
        let file_name: String = row.get(offset + 1)?;
        let is_dir = row.get_ref(offset + 2)?.as_bytes_or_null()?.is_some();

        //let is_dir = FileViewInfo::is_dir(&file_name_db);
        Ok(FileViewInfo {
            id: row.get(offset)?,
            is_selected: false,
            icon: FileViewInfo::get_icon(is_dir, &file_name, row.get(offset + 6)?),
            is_dir,
            file_name,
            file_size: row.get(offset + 3)?,
            file_size_pretty: FileViewInfo::pretty_size(row.get(offset + 3)?),
            created: row.get(offset + 4)?,
            modified: row.get(offset + 5)?,
        })
    }
}

impl TableFile<FileView> {
    pub fn get_in_virtual_dir(
        db: &Connection,
        user_id: i64,
        dir: String,
    ) -> Result<FileView, rusqlite::Error> {
        let dir_result: Result<FileViewInfo, rusqlite::Error> = db
            .prepare(
                "SELECT
                        id,
                        file_name_virt,
                        file_path_virt,
                        file_size,
                        created,
                        modified,
                        icon
                    FROM file
                    WHERE
                        user_id = ?1 AND file_path_virt = ?2",
            )?
            .query_one(params![user_id, dir], |row| TableFile::fill(row, 0));

        match dir_result {
            Ok(dir) => {
                let mut children_result = db
                    .prepare(
                        "SELECT
                                id,
                                file_name_virt,
                                file_path_virt,
                                file_size,
                                created,
                                modified,
                                icon
                            FROM file
                            WHERE
                                parent_id = ?1",
                    )?
                    .query_map(params![dir.id], |row| TableFile::fill(row, 0))?
                    .collect::<Result<Vec<FileViewInfo>, rusqlite::Error>>()?;

                FileViewInfo::sort(&mut children_result);

                Ok(FileView {
                    current_dir: Some(dir),
                    children: children_result,
                })
            }
            Err(e) => match e {
                rusqlite::Error::QueryReturnedNoRows => Ok(FileView {
                    // Directory not found
                    current_dir: None,
                    children: vec![],
                }),
                _ => Err(e), // Return error
            },
        }
    }
}
