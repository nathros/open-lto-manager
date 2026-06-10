use std::marker::PhantomData;

use rusqlite::{Connection, params};

use crate::{
    backend::database::tables::table::{
        RecordDelete, RecordFill, RecordInsert, RecordRead, RecordUpdate, TableClear, TableCreate,
        TableUpdate,
    },
    shared::models::database::file::model_file::RecordFile,
};

pub struct TableFile<T = RecordFile> {
    phantom: PhantomData<T>,
}

impl TableCreate<RecordFile> for TableFile<RecordFile> {
    fn create_table(db: &Connection) -> Result<bool, rusqlite::Error> {
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
                hash TEXT,
                icon TEXT,
                FOREIGN KEY(tape_id) REFERENCES tape(id),
                CONSTRAINT path_name_pair UNIQUE(file_name_virt, file_path_virt)
            );",
            (),
        )?;

        db.execute("CREATE INDEX v_path ON file(file_path_virt);", ())?;
        db.execute("CREATE INDEX p_path ON file(file_path_phy);", ())?;

        TableFile::insert(db, &RecordFile::root_dir())?;

        Ok(true)
    }
}

impl TableUpdate<RecordFile> for TableFile<RecordFile> {
    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, rusqlite::Error> {
        Ok(false)
    }
}

impl RecordRead<RecordFile> for TableFile<RecordFile> {
    fn get(db: &Connection, record_id: i64) -> Result<RecordFile, rusqlite::Error> {
        db.prepare(
            "SELECT
                    id,
                    tape_id,
                    file_name_virt,
                    file_path_virt,
                    file_name_phy,
                    file_path_phy,
                    file_size,
                    created,
                    modified,
                    hash,
                    icon
                FROM file
                WHERE id = ?1",
        )?
        .query_one([record_id], |row| TableFile::fill(row, 0))
    }
}

impl RecordInsert<RecordFile> for TableFile<RecordFile> {
    fn insert(db: &Connection, record: &RecordFile) -> Result<i64, rusqlite::Error> {
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
                    hash,
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
                record.hash,
                record.icon,
            ],
        )?;
        Ok(db.last_insert_rowid())
    }
}

impl RecordUpdate<RecordFile> for TableFile<RecordFile> {
    fn update(db: &Connection, record: &RecordFile) -> Result<usize, rusqlite::Error> {
        db.execute(
            "UPDATE file SET
                    file_name_virt = ?1,
                    file_path_virt = ?2,
                    file_name_phy = ?3,
                    file_path_phy = ?4,
                    file_size = ?5,
                    created = ?6,
                    modified = ?7,
                    hash = ?8,
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
                record.hash,
                record.icon,
                record.id
            ],
        )
    }
}

impl RecordDelete<RecordFile> for TableFile<RecordFile> {
    fn delete(db: &Connection, record_id: i64) -> Result<usize, rusqlite::Error> {
        db.execute("DELETE FROM file WHERE id = ?1;", params![record_id])
    }
}

impl TableClear<RecordFile> for TableFile<RecordFile> {
    fn clear_table(db: &Connection) -> Result<usize, rusqlite::Error> {
        db.execute("DELETE FROM file;", ())
    }
}

impl RecordFill<RecordFile> for TableFile<RecordFile> {
    fn fill(row: &rusqlite::Row<'_>, offset: usize) -> Result<RecordFile, rusqlite::Error> {
        Ok(RecordFile {
            id: row.get(offset)?,
            tape_id: row.get(offset + 1)?,
            file_name_virt: row.get(offset + 2)?,
            file_path_virt: row.get(offset + 3)?,
            file_name_phy: row.get(offset + 4)?,
            file_path_phy: row.get(offset + 5)?,
            file_size: row.get(offset + 6)?,
            created: row.get(offset + 7)?,
            modified: row.get(offset + 8)?,
            hash: row.get(offset + 9)?,
            icon: row.get(offset + 10)?,
        })
    }
}

impl TableFile<RecordFile> {
    pub fn get_all(db: &Connection) -> Result<Vec<RecordFile>, rusqlite::Error> {
        db.prepare(
            "SELECT
                    id,
                    tape_id,
                    file_name_virt,
                    file_path_virt,
                    file_name_phy,
                    file_path_phy,
                    file_size,
                    created,
                    modified,
                    hash,
                    icon
                FROM file
                ORDER BY id",
        )?
        .query_map([], |row| TableFile::fill(row, 0))?
        .collect::<Result<Vec<RecordFile>, rusqlite::Error>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::database::tables::{file::table_file::TableFile, table::TableCreate, tape};

    fn create_table() -> rusqlite::Connection {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        // TableFile depends on TableManufacturer, TableTapeType and TableTape, so these must be created first
        tape::table_tape::tests::create_table(&conn); // This creates TableManufacturer and TableTapeType

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
    fn create() {
        let _db = create_table();
    }

    #[test]
    fn insert() {
        let _db = create_table();
    }
}
