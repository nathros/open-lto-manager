use rusqlite::{params, Connection, Error};

use crate::backend::database::{
    models::model_file::{RecordFile, RecordFileJoin},
    tables::table::Table,
};

pub struct TableFile {}

impl Table<RecordFile, RecordFileJoin> for TableFile {
    fn create_table(db: &Connection) -> Result<bool, Error> {
        match db.table_exists(None, "file") {
            std::result::Result::Ok(exist) => {
                if exist == true {
                    return Ok(false);
                }
            }
            Err(e) => return Err(e),
        }

        if let Err(e) = db.execute(
            "CREATE TABLE IF NOT EXISTS tape (
                id INTEGER PRIMARY KEY,
                tape_id INTEGER NOT NULL,
                file_name_virt TEXT NOT NULL,
                file_path_virt TEXT NOT NULL,
                file_name_phy TEXT NOT NULL,
                file_path_phy TEXT NOT NULL,
                file_size INTEGER NOT NULL,
                created BIGINT NOT NULL,
                modified BIGINT NOT NULL,
                crc32 TEXT NOT NULL,
                icon TEXT,
                FOREIGN KEY(tape_id) REFERENCES tape(id)
            );",
            (),
        ) {
            return Err(e); // Failed to create table
        }
        return Ok(true);
    }

    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, Error> {
        Ok(false)
    }

    fn get(_db: &Connection, _record_id: i64) -> Result<RecordFile, Error> {
        todo!()
    }

    fn get_join(_db: &Connection, _record_id: i64) -> Result<RecordFileJoin, Error> {
        todo!()
    }

    fn insert_record(db: &Connection, record: &RecordFile) -> Result<usize, Error> {
        db.execute(
            "INSERT INTO tape (
                    tape_id,
                    file_name_virt,
                    file_path_virt,
                    file_name_phy,
                    file_path_phy,
                    file_size,
                    created,
                    modified,
                    crc32,
                    icon)
                VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4,
                    ?5,
                    ?6,
                    ?7,
                    ?8,
                    ?9,
                    ?10);",
            params![
                record.tape_id,
                record.file_name_virt,
                record.file_path_virt,
                record.file_name_phy,
                record.file_path_phy,
                record.file_size,
                record.created,
                record.modified,
                record.crc32,
                record.icon,
            ],
        )
    }

    fn update_record(db: &Connection, record: &RecordFile) -> Result<usize, Error> {
        db.execute(
            "UPDATE tape SET
                    tape_id = ?1,
                    file_name_virt = ?2,
                    file_path_virt = ?3,
                    file_name_phy = ?4,
                    file_path_phy = ?5,
                    file_size = ?6,
                    created = ?7,
                    modified = ?8,
                    crc32 = ?9,
                    icon = ?10,
                WHERE id = ?11",
            params![
                record.tape_id,
                record.file_name_virt,
                record.file_path_virt,
                record.file_name_phy,
                record.file_path_phy,
                record.file_size,
                record.created,
                record.modified,
                record.crc32,
                record.icon,
                record.id
            ],
        )
    }

    fn delete_record(db: &Connection, record_id: i64) -> Result<usize, Error> {
        db.execute("DELETE FROM file WHERE id = ?1;", params![record_id])
    }

    fn fill(_row: &rusqlite::Row<'_>, _offset: usize) -> Result<RecordFile, Error> {
        todo!()
    }
}

/*impl TableTape {
    pub fn get_all(db: &Connection) -> Result<Vec<RecordManufacturer>, rusqlite::Error> {
        db.prepare(
            "SELECT id, name FROM manufacturer ORDER BY
                    CASE id
                        WHEN 1 THEN 2
                    END,
                    name", // Order by name then "Other" [id=1] to be last
        )?
        .query_map([], |row| {
            Ok(RecordManufacturer {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<RecordManufacturer>, rusqlite::Error>>()
    }
}*/
