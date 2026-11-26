use chrono::Local;
use rusqlite::{Connection, params};
use tracing::info;

use crate::{
    backend::database::{
        db::database_generic_error,
        tables::{file::table_file::TableFile, table::RecordFill},
    },
    shared::{
        models::file_view::{FileView, FileViewInfo},
        validator::{validator_base::TValidator, validators::Validators},
    },
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

    pub fn add_new_dir(
        db: &Connection,
        user_id: i64,
        name: String,
        parent: String,
    ) -> Result<bool, rusqlite::Error> {
        if let Some(error) = Validators::virtual_dir_name(
            vec![], // Duplicates handled by database UNIQUE constraint
        )
        .validate(&name)
        {
            return Err(database_generic_error(error));
        }

        let new_path = [parent.as_str(), name.as_str(), "/"].concat();
        let now = Local::now();
        info!(
            "Add new virtual dir: parent {}, name {}, new_path {}",
            parent, name, new_path
        );
        Ok(db.execute(
            "WITH const AS (
                    SELECT id FROM file
                    WHERE file_path_virt = ?1 AND user_id = ?2
                )
                INSERT INTO file (
                    parent_id,
                    user_id,
                    tape_id,
                    file_name_virt,
                    file_path_virt,
                    file_name_phy,
                    file_path_phy,
                    file_size,
                    created,
                    modified,
                    hash,
                    icon)
                SELECT
                    (SELECT id FROM const),
                    ?2,
                    NULL,
                    ?3,
                    ?4,
                    \"\",
                    \"\",
                    0,
                    ?5,
                    ?5,
                    \"\",
                    \"\"
                WHERE EXISTS (SELECT id FROM const);",
            params![parent, user_id, name, new_path, now],
        )? > 0)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        backend::database::{
            db::tests::create_test_database,
            tables::{file::table_file::TableFile, tape, user::table_user::tests::any_user},
        },
        shared::models::{database::file::model_file::RecordFile, file_view::FileView},
    };

    #[test]
    fn new_dir_add_invalid() {
        let conn = create_test_database();
        tape::table_tape::tests::create_table(&conn);
        let user = any_user(&conn);
        let user_root_dir = RecordFile::root_dir(user.id).file_path_virt.unwrap();

        assert!(
            TableFile::<FileView>::add_new_dir(
                &conn,
                user.id,
                "/invalid".to_string(),
                user_root_dir.clone(),
            )
            .is_err()
        );
        assert!(
            TableFile::<FileView>::add_new_dir(
                &conn,
                user.id,
                "long_long_long_long_long_long_long_long_long_long_long_long_long_long".to_string(),
                user_root_dir.clone(),
            )
            .is_err()
        );
    }

    #[test]
    fn new_dir_add_nested() {
        let conn = create_test_database();
        tape::table_tape::tests::create_table(&conn);
        let user = any_user(&conn);
        let user_root_dir = RecordFile::root_dir(user.id);

        {
            let user_virt_files = TableFile::<FileView>::get_in_virtual_dir(
                &conn,
                user.id,
                user_root_dir.file_name_virt.clone(),
            )
            .unwrap();

            // Check user root dir is set and valid
            assert!(user_virt_files.current_dir.is_some()); // Root dir exists
            if let Some(root_dir) = user_virt_files.current_dir {
                assert!(root_dir.is_dir);
                assert_eq!(root_dir.file_name, user_root_dir.file_name_virt);
            }
            assert!(
                user_virt_files.children.is_empty(),
                "Expected no children to root dir for new user"
            );
        }

        // Add new child dir to root
        let dir1 = "nested".to_string();
        let dir1_path = "/nested/".to_string();
        let dir2 = "test inner".to_string();
        assert!(
            TableFile::<FileView>::add_new_dir(
                &conn,
                user.id,
                dir1.clone(),
                user_root_dir.file_path_virt.as_ref().unwrap().clone(),
            )
            .unwrap(),
            "Failed to add new dir"
        );
        // Add again should fail
        assert!(
            TableFile::<FileView>::add_new_dir(
                &conn,
                user.id,
                dir1.clone(),
                user_root_dir.file_path_virt.unwrap().clone(),
            )
            .is_err(),
            "Should not have been able to add same dir again"
        );

        {
            // Refetch root dir
            let user_virt_files = TableFile::<FileView>::get_in_virtual_dir(
                &conn,
                user.id,
                user_root_dir.file_name_virt.clone(),
            )
            .unwrap();
            assert!(
                user_virt_files.children.len() == 1,
                "Expected 1 child to root dir for new user"
            );

            let new_child = user_virt_files.children.first().unwrap();
            assert!(new_child.is_dir);
            assert_eq!(new_child.file_name, dir1);

            // Add nested dir of new dir
            {
                assert!(
                    TableFile::<FileView>::add_new_dir(
                        &conn,
                        user.id,
                        dir2.clone(),
                        dir1_path.clone()
                    )
                    .unwrap(),
                    "Failed to add second new dir"
                );
                assert!(
                    TableFile::<FileView>::add_new_dir(
                        &conn,
                        user.id,
                        dir2.clone(),
                        dir1_path.clone()
                    )
                    .is_err(),
                    "Should not have been able to add same dir again"
                );
                let user_virt_files_nested =
                    TableFile::<FileView>::get_in_virtual_dir(&conn, user.id, dir1_path.clone())
                        .unwrap();

                let new_child_inner = user_virt_files_nested.children.first().unwrap();
                assert!(new_child_inner.is_dir);
                assert_eq!(new_child_inner.file_name, dir2);
                assert!(user_virt_files_nested.current_dir.is_some());

                //TableFile::<RecordFile>::get_all(&conn)
                //    .unwrap()
                //    .iter()
                //    .for_each(|f| println!("{:?}", f));
            }
        }
    }
}
