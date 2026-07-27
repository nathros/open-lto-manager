use std::marker::PhantomData;

use rusqlite::{Connection, params};

use crate::{
    backend::database::tables::table::{
        RecordDelete, RecordFill, RecordInsert, RecordRead, RecordUpdate, TableCreate, TableUpdate,
    },
    shared::models::database::tape::model_tape::RecordTape,
};

pub struct TableTape<T = RecordTape> {
    phantom: PhantomData<T>,
}

impl TableCreate<RecordTape> for TableTape<RecordTape> {
    fn create_table(db: &Connection) -> Result<bool, rusqlite::Error> {
        match db.table_exists(None, "tape") {
            std::result::Result::Ok(exist) => {
                if exist {
                    return Ok(false);
                }
            }
            Err(e) => return Err(e),
        }

        db.execute(
            "CREATE TABLE IF NOT EXISTS tape (
                id INTEGER PRIMARY KEY,
                manufacturer_id INTEGER NOT NULL,
                tape_type_id INTEGER NOT NULL,
                barcode VARCHAR(8) UNIQUE,
                serial TEXT UNIQUE,
                format INTEGER NOT NULL,
                worm BOOLEAN NOT NULL,
                encryption_type INTEGER NOT NULL,
                encryption_sw INTEGER NOT NULL,
                encryption_hw INTEGER NOT NULL,
                compressed BOOLEAN NOT NULL,
                used_space INTEGER NOT NULL,
                created BIGINT NOT NULL,
                last_used BIGINT NOT NULL,
                FOREIGN KEY(manufacturer_id) REFERENCES manufacturer(id),
                FOREIGN KEY(tape_type_id) REFERENCES tape_type(id)
            );",
            (),
        )?;
        Ok(true)
    }
}

impl TableUpdate<RecordTape> for TableTape<RecordTape> {
    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, rusqlite::Error> {
        Ok(false)
    }
}

impl RecordRead<RecordTape> for TableTape<RecordTape> {
    fn get(db: &Connection, record_id: i64) -> Result<RecordTape, rusqlite::Error> {
        db.prepare(
            "SELECT
                    id,
                    manufacturer_id,
                    tape_type_id,
                    barcode,
                    serial,
                    format,
                    worm,
                    encryption_type,
                    encryption_sw,
                    encryption_hw,
                    compressed,
                    used_space,
                    created,
                    last_used
                FROM tape
                WHERE id = ?1",
        )?
        .query_one([record_id], |row| TableTape::fill(row, 0))
    }
}

impl RecordInsert<RecordTape> for TableTape<RecordTape> {
    fn insert(db: &Connection, record: &RecordTape) -> Result<i64, rusqlite::Error> {
        db.execute(
            "INSERT INTO tape (
                    id,
                    manufacturer_id,
                    tape_type_id,
                    barcode,
                    serial,
                    format,
                    worm,
                    encryption_type,
                    encryption_sw,
                    encryption_hw,
                    compressed,
                    used_space,
                    created,
                    last_used)
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
                    ?10,
                    ?11,
                    ?12,
                    ?13,
                    ?14);",
            params![
                record.id,
                record.manufacturer_id,
                record.tape_type_id,
                record.barcode,
                record.serial,
                record.format,
                record.worm,
                record.encryption_type,
                record.encryption_sw,
                record.encryption_hw,
                record.compressed,
                record.used_space,
                record.created,
                record.last_used
            ],
        )?;
        Ok(db.last_insert_rowid())
    }
}

impl RecordUpdate<RecordTape> for TableTape<RecordTape> {
    fn update(db: &Connection, record: &RecordTape) -> Result<usize, rusqlite::Error> {
        db.execute(
            "UPDATE tape SET
                    manufacturer_id = ?1,
                    tape_type_id = ?2,
                    barcode = ?3,
                    serial = ?4,
                    format = ?5,
                    worm = ?6,
                    encryption_type = ?7,
                    encryption_sw = ?8,
                    encryption_hw = ?9,
                    compressed = ?10,
                    used_space = ?11,
                    created = ?12,
                    last_used = ?13
                WHERE id = ?14",
            params![
                record.manufacturer_id,
                record.tape_type_id,
                record.barcode,
                record.serial,
                record.format,
                record.worm,
                record.encryption_type,
                record.encryption_sw,
                record.encryption_hw,
                record.compressed,
                record.used_space,
                record.created,
                record.last_used,
                record.id
            ],
        )
    }
}

impl RecordDelete<RecordTape> for TableTape<RecordTape> {
    fn delete(db: &Connection, record_id: i64) -> Result<usize, rusqlite::Error> {
        db.execute("DELETE FROM tape WHERE id = ?1;", params![record_id])
    }
}

impl RecordFill<RecordTape> for TableTape<RecordTape> {
    fn fill(row: &rusqlite::Row<'_>, offset: usize) -> Result<RecordTape, rusqlite::Error> {
        Ok(RecordTape {
            id: row.get(offset)?,
            manufacturer_id: row.get(offset + 1)?,
            tape_type_id: row.get(offset + 2)?,
            barcode: row.get(offset + 3)?,
            serial: row.get(offset + 4)?,
            format: row.get(offset + 5)?,
            worm: row.get(offset + 6)?,
            encryption_type: row.get(offset + 7)?,
            encryption_sw: row.get(offset + 8)?,
            encryption_hw: row.get(offset + 9)?,
            compressed: row.get(offset + 10)?,
            used_space: row.get(offset + 11)?,
            created: row.get(offset + 12)?,
            last_used: row.get(offset + 13)?,
        })
    }
}

impl TableTape<RecordTape> {
    pub fn get_all(db: &Connection) -> Result<Vec<RecordTape>, rusqlite::Error> {
        db.prepare(
            "SELECT
                    id,
                    manufacturer_id,
                    tape_type_id,
                    barcode,
                    serial,
                    format,
                    worm,
                    encryption_type,
                    encryption_sw,
                    encryption_hw,
                    compressed,
                    used_space,
                    created,
                    last_used
             FROM tape;",
        )?
        .query_map([], |row| TableTape::fill(row, 0))?
        .collect::<Result<Vec<RecordTape>, rusqlite::Error>>()
    }
}

#[cfg(test)]
pub mod tests {
    use chrono::Local;

    use crate::{
        backend::database::{
            db::tests::create_test_database,
            tables::{
                manufacturer::{self, table_manufacturer::TableManufacturer},
                table::{RecordDelete, RecordInsert, RecordRead, RecordUpdate, TableCreate},
                tape::table_tape::TableTape,
                tape_type::{self, table_tape_type::TableTapeType},
            },
        },
        shared::models::database::tape::model_tape::{
            EncryptionType, HardwareEncryptionType, RecordTape, SoftwareEncryptionType, TapeFormat,
        },
    };

    pub fn create_table(conn: &rusqlite::Connection) {
        // TableTape depends on TableManufacturer and TableTapeType, so these must be created first
        manufacturer::table_manufacturer::tests::create_table(conn);
        tape_type::table_tape_type::tests::create_table(conn);

        assert!(
            !conn.table_exists(None, "tape").unwrap(),
            "New table should be empty"
        );
        assert!(
            TableTape::create_table(conn).is_ok(),
            "Failed to create tape table"
        );
        assert!(
            conn.table_exists(None, "tape").unwrap(),
            "create_table() tape reported Ok but table does not exist"
        );
    }

    fn insert(db: &rusqlite::Connection) -> i64 {
        let manufacturers = TableManufacturer::get_all(db).unwrap();
        let types = TableTapeType::get_all(db).unwrap();
        let mut new_tape = RecordTape {
            id: 0,
            manufacturer_id: manufacturers.get(manufacturers.len() / 2).unwrap().id,
            tape_type_id: types.get(types.len() / 2).unwrap().id,
            barcode: "test barcode".to_string(),
            serial: "test serial".to_string(),
            format: TapeFormat::Tar,
            worm: false,
            encryption_type: EncryptionType::Software,
            encryption_sw: SoftwareEncryptionType::Test,
            encryption_hw: HardwareEncryptionType::None,
            compressed: true,
            used_space: 1234,
            created: Local::now(),
            last_used: Local::now(),
        };
        let new_id = TableTape::insert(db, &new_tape).unwrap();

        let inserted_tape = TableTape::get(db, new_id).unwrap();
        new_tape.id = inserted_tape.id;
        assert_eq!(new_tape, inserted_tape, "Inserted tape does not match");
        new_tape.id
    }

    fn update(db: &rusqlite::Connection, test_record_id: i64) {
        let test_tape = TableTape::get(db, test_record_id).unwrap();

        let changed_tape = RecordTape {
            id: test_tape.id,
            manufacturer_id: test_tape.manufacturer_id + 1,
            tape_type_id: test_tape.tape_type_id + 1,
            barcode: format!("Added: {}", test_tape.barcode),
            serial: format!("Added: {}", test_tape.serial),
            format: match test_tape.format {
                TapeFormat::Tar => TapeFormat::LTFS,
                TapeFormat::LTFS => TapeFormat::Tar,
            },
            worm: !test_tape.worm,
            encryption_type: match test_tape.encryption_type {
                EncryptionType::None => EncryptionType::Hardware,
                EncryptionType::Software => EncryptionType::None,
                EncryptionType::Hardware => EncryptionType::Software,
            },
            encryption_sw: match test_tape.encryption_sw {
                SoftwareEncryptionType::None => SoftwareEncryptionType::Test,
                SoftwareEncryptionType::Test => SoftwareEncryptionType::None,
            },
            encryption_hw: match test_tape.encryption_hw {
                HardwareEncryptionType::None => HardwareEncryptionType::Test,
                HardwareEncryptionType::Test => HardwareEncryptionType::None,
            },
            compressed: !test_tape.compressed,
            used_space: test_tape.used_space + 100,
            created: Local::now(),
            last_used: Local::now(),
        };

        TableTape::update(db, &changed_tape).unwrap();

        let updated_tape = TableTape::get(db, test_record_id).unwrap();
        assert_eq!(updated_tape, changed_tape, "Update record failure");
    }

    fn delete(db: &rusqlite::Connection, test_record_id: i64) {
        assert!(
            TableTape::get(db, test_record_id).is_ok(),
            "Test record does not exist"
        );
        TableTape::delete(db, test_record_id).unwrap();
        assert!(
            TableTape::get(db, test_record_id).is_err(),
            "Failed to delete"
        );
    }

    #[test]
    fn suite() {
        let conn = create_test_database();
        create_table(&conn);
        let new_id = insert(&conn);
        update(&conn, new_id);
        delete(&conn, new_id);
    }
}
