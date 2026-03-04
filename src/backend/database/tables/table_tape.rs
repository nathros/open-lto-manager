use rusqlite::{Connection, Error, params};

use crate::shared::models::database::model_tape::{RecordTape, RecordTapeJoin};

use super::table::Table;

pub struct TableTape {}

impl Table<RecordTape, RecordTapeJoin> for TableTape {
    fn create_table(db: &Connection) -> Result<bool, Error> {
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

    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, Error> {
        Ok(false)
    }

    fn get(db: &Connection, record_id: i64) -> Result<RecordTape, Error> {
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

    fn get_join(_db: &Connection, _record_id: i64) -> Result<RecordTapeJoin, Error> {
        todo!()
    }

    fn insert_record(db: &Connection, record: &RecordTape) -> Result<i64, Error> {
        db.execute(
            "INSERT INTO tape (
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
                    ?13);",
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
                record.last_used
            ],
        )?;
        Ok(db.last_insert_rowid())
    }

    fn insert_batch(db: &Connection, records: &[RecordTape]) -> Result<usize, Error> {
        let mut count = 0;
        let mut prepared = db.prepare(
            "INSERT INTO tape (
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
                    ?13);",
        )?;
        for record in records {
            count += prepared.execute(params![
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
            ])?;
        }
        Ok(count)
    }

    fn update_record(db: &Connection, record: &RecordTape) -> Result<usize, Error> {
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

    fn delete_record(db: &Connection, record_id: i64) -> Result<usize, Error> {
        db.execute("DELETE FROM tape WHERE id = ?1;", params![record_id])
    }

    fn fill(row: &rusqlite::Row<'_>, offset: usize) -> Result<RecordTape, Error> {
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

impl TableTape {
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
mod tests {
    #![allow(clippy::unwrap_used)]

    use chrono::Local;

    use crate::{
        backend::database::tables::{
            table::Table, table_manufacturer::TableManufacturer, table_tape::TableTape,
            table_tape_type::TableTapeType,
        },
        shared::models::database::model_tape::{
            EncryptionType, HardwareEncryptionType, RecordTape, SoftwareEncryptionType, TapeFormat,
        },
    };

    fn create() -> rusqlite::Connection {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        // TableTape depends on TableManufacturer and TableTapeType, so these must be created first
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
            "New table should be empty"
        );
        assert!(
            TableTape::create_table(&conn).is_ok(),
            "Failed to create tape table"
        );
        assert!(
            conn.table_exists(None, "tape").unwrap(),
            "create_table() tape reported Ok but table does not exist"
        );
        conn
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
        TableTape::insert_record(db, &new_tape).unwrap();

        let inserted_tape = TableTape::get(db, 1).unwrap();
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

        TableTape::update_record(db, &changed_tape).unwrap();

        let updated_tape = TableTape::get(db, test_record_id).unwrap();
        assert_eq!(updated_tape, changed_tape, "Update record failure");
    }

    fn delete(db: &rusqlite::Connection, test_record_id: i64) {
        assert!(
            TableTape::get(db, test_record_id).is_ok(),
            "Test record does not exist"
        );
        TableTape::delete_record(db, test_record_id).unwrap();
        assert!(
            TableTape::get(db, test_record_id).is_err(),
            "Failed to delete"
        );
    }

    #[test]
    fn suite() {
        let db = create();
        let new_id = insert(&db);
        update(&db, new_id);
        delete(&db, new_id);
    }
}
