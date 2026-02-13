use rusqlite::{Connection, Error, params};

use crate::{
    backend::database::tables::table::Table,
    shared::models::database::model_file::{RecordFile, RecordFileJoin},
};

pub struct TableFile {}

impl Table<RecordFile, RecordFileJoin> for TableFile {
    fn create_table(db: &Connection) -> Result<bool, Error> {
        match db.table_exists(None, "file") {
            std::result::Result::Ok(exist) => {
                if exist {
                    return Ok(false);
                }
            }
            Err(e) => return Err(e),
        }

        db.execute(
            "CREATE TABLE IF NOT EXISTS file (
                id INTEGER PRIMARY KEY,
                tape_id INTEGER,
                file_name_virt TEXT NOT NULL,
                file_path_virt TEXT NOT NULL,
                file_name_phy TEXT NOT NULL,
                file_path_phy TEXT NOT NULL,
                file_size INTEGER NOT NULL,
                created BIGINT NOT NULL,
                modified BIGINT NOT NULL,
                crc32 TEXT NOT NULL,
                icon TEXT,
                FOREIGN KEY(tape_id) REFERENCES tape(id),
                CONSTRAINT path_name_pair UNIQUE(file_name_virt, file_path_virt)
            );",
            (),
        )?;

        db.execute("CREATE INDEX v_path ON file(file_path_virt);", ())?;
        db.execute("CREATE INDEX p_path ON file(file_path_phy);", ())?;

        Self::insert_record(db, &RecordFile::root_dir())?;

        Ok(true)
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
            "INSERT INTO file (
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
            "UPDATE file SET
                    file_name_virt = ?1,
                    file_path_virt = ?2,
                    file_name_phy = ?3,
                    file_path_phy = ?4,
                    file_size = ?5,
                    created = ?6,
                    modified = ?7,
                    crc32 = ?8,
                    icon = ?9,
                WHERE id = ?10",
            params![
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

impl TableFile {}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use crate::backend::database::tables::{
        table::Table, table_file::TableFile, table_manufacturer::TableManufacturer,
        table_tape::TableTape, table_tape_type::TableTapeType,
    };

    fn create() -> rusqlite::Connection {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        // TableFile depends on TableManufacturer, TableTapeType and TableTape, so these must be created first
        assert!(
            !conn.table_exists(None, "manufacturer").unwrap(),
            "New table manufacturer should be empty"
        );
        assert!(
            TableManufacturer::create_table(&conn).is_ok(),
            "Failed to create manufacturer table"
        );
        assert!(
            conn.table_exists(None, "manufacturer").unwrap(),
            "create_table() manufacturer reported Ok but table does not exist"
        );

        assert!(
            !conn.table_exists(None, "tape_type").unwrap(),
            "New table tape_type should be empty"
        );
        assert!(
            TableTapeType::create_table(&conn).is_ok(),
            "Failed to create tape_type table"
        );
        assert!(
            conn.table_exists(None, "tape_type").unwrap(),
            "create_table() tape_type reported Ok but table does not exist"
        );

        assert!(
            !conn.table_exists(None, "tape").unwrap(),
            "New table tape_type should be empty"
        );
        assert!(
            TableTape::create_table(&conn).is_ok(),
            "Failed to create tape table"
        );
        assert!(
            conn.table_exists(None, "tape").unwrap(),
            "create_table() tape reported Ok but table does not exist"
        );

        assert!(
            !conn.table_exists(None, "file").unwrap(),
            "New table should be empty"
        );
        assert!(
            TableFile::create_table(&conn).is_ok(),
            "Failed to create file table"
        );
        assert!(
            conn.table_exists(None, "file").unwrap(),
            "create_table() file reported Ok but table does not exist"
        );
        conn
    }

    #[test]
    fn suite() {
        let _db = create();
    }
}
