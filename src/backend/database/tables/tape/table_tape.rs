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
                user_id INTEGER NOT NULL,
                manufacturer_id INTEGER NOT NULL,
                tape_type_id INTEGER NOT NULL,
                barcode VARCHAR(8),
                serial TEXT,
                format INTEGER NOT NULL,
                worm BOOLEAN NOT NULL,
                encryption_type INTEGER NOT NULL,
                encryption_sw INTEGER NOT NULL,
                encryption_hw INTEGER NOT NULL,
                compressed BOOLEAN NOT NULL,
                used_space INTEGER NOT NULL,
                created BIGINT NOT NULL,
                last_used BIGINT NOT NULL,
                FOREIGN KEY(user_id) REFERENCES user(id),
                FOREIGN KEY(manufacturer_id) REFERENCES manufacturer(id),
                FOREIGN KEY(tape_type_id) REFERENCES tape_type(id),
                UNIQUE(user_id, barcode),
                UNIQUE(user_id, serial)
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
                    user_id,
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
                    user_id,
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
                record.user_id,
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
            user_id: row.get(offset + 1)?,
            manufacturer_id: row.get(offset + 2)?,
            tape_type_id: row.get(offset + 3)?,
            barcode: row.get(offset + 4)?,
            serial: row.get(offset + 5)?,
            format: row.get(offset + 6)?,
            worm: row.get(offset + 7)?,
            encryption_type: row.get(offset + 8)?,
            encryption_sw: row.get(offset + 9)?,
            encryption_hw: row.get(offset + 10)?,
            compressed: row.get(offset + 11)?,
            used_space: row.get(offset + 12)?,
            created: row.get(offset + 13)?,
            last_used: row.get(offset + 14)?,
        })
    }
}

impl TableTape<RecordTape> {
    pub fn get_all(db: &Connection) -> Result<Vec<RecordTape>, rusqlite::Error> {
        db.prepare(
            "SELECT
                    id,
                    user_id,
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

    pub fn barcode_exists(
        db: &Connection,
        user_id: i64,
        barcode: String,
    ) -> Result<bool, rusqlite::Error> {
        let count: i64 = db.query_row(
            "SELECT COUNT(barcode)
                    FROM tape
                    WHERE user_id =?1 AND barcode = ?2;",
            params![user_id, barcode],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn serial_exists(
        db: &Connection,
        user_id: i64,
        serial: String,
    ) -> Result<bool, rusqlite::Error> {
        let count: i64 = db.query_row(
            "SELECT COUNT(serial)
                    FROM tape
                    WHERE user_id =?1 AND serial = ?2;",
            params![user_id, serial],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }
}

#[cfg(test)]
pub mod tests {
    use chrono::Local;
    const TEST_BARCODE: &str = "test barcode";
    const UPDATED_BARCODE: &str = "test barcode changed";
    const TEST_SERIAL: &str = "test serial";
    const UPDATED_SERIAL: &str = "test serial changed";

    use crate::{
        backend::database::{
            db::tests::create_test_database,
            tables::{
                file,
                manufacturer::{self, table_manufacturer::TableManufacturer},
                table::{RecordDelete, RecordInsert, RecordRead, RecordUpdate, TableCreate},
                tape::table_tape::TableTape,
                tape_type::{self, table_tape_type::TableTapeType},
                user::{
                    self,
                    table_user::{TableUser, tests::create_additional_user},
                },
            },
        },
        shared::models::database::{
            tape::model_tape::{
                EncryptionType, HardwareEncryptionType, RecordTape, SoftwareEncryptionType,
                TapeFormat,
            },
            tape_type::model_tape_type::RecordTapeType,
            user::model_user_sensitive::RecordUser,
        },
    };

    pub fn create_table(conn: &rusqlite::Connection) {
        // TableTape depends on TableUser, TableManufacturer and TableTapeType, so these must be created first
        user::table_user::tests::create_table(conn);
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

        file::table_file::tests::create_table(conn); // This will create default user
    }

    fn insert(db: &rusqlite::Connection, user_id: i64) -> i64 {
        let manufacturers = TableManufacturer::get_all(db).unwrap();
        let types = TableTapeType::<RecordTapeType>::get_all(db).unwrap();
        let mut new_tape = RecordTape {
            id: 0,
            user_id,
            manufacturer_id: manufacturers.get(manufacturers.len() / 2).unwrap().id,
            tape_type_id: types.get(types.len() / 2).unwrap().id,
            barcode: TEST_BARCODE.to_string(),
            serial: Some(TEST_SERIAL.to_string()),
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
            user_id: test_tape.user_id,
            manufacturer_id: test_tape.manufacturer_id + 1,
            tape_type_id: test_tape.tape_type_id + 1,
            barcode: UPDATED_BARCODE.to_string(),
            serial: Some(UPDATED_SERIAL.to_string()),
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

    fn check_duplicate_barcode(
        db: &rusqlite::Connection,
        user_id: i64,
        barcode: String,
        expected: bool,
    ) {
        assert_eq!(
            TableTape::barcode_exists(db, user_id, barcode).unwrap(),
            expected
        );
    }

    fn check_duplicate_serial(
        db: &rusqlite::Connection,
        user_id: i64,
        serial: String,
        expected: bool,
    ) {
        assert_eq!(
            TableTape::serial_exists(db, user_id, serial).unwrap(),
            expected
        );
    }

    #[test]
    fn suite() {
        let conn = create_test_database();
        create_table(&conn);
        let first_user = TableUser::<RecordUser>::get_all(&conn)
            .unwrap()
            .first()
            .unwrap()
            .clone();
        let second_user = create_additional_user(&conn);

        // Barcode and serial does not exist for either user
        check_duplicate_barcode(&conn, first_user.id, TEST_BARCODE.to_string(), false);
        check_duplicate_serial(&conn, first_user.id, TEST_SERIAL.to_string(), false);
        check_duplicate_barcode(&conn, second_user.id, TEST_BARCODE.to_string(), false);
        check_duplicate_serial(&conn, second_user.id, TEST_SERIAL.to_string(), false);

        let new_id = insert(&conn, first_user.id); // Insert new Tape as first user
        // Expect match for first user
        check_duplicate_barcode(&conn, first_user.id, TEST_BARCODE.to_string(), true);
        check_duplicate_serial(&conn, first_user.id, TEST_SERIAL.to_string(), true);
        check_duplicate_barcode(&conn, first_user.id, UPDATED_BARCODE.to_string(), false);
        check_duplicate_serial(&conn, first_user.id, UPDATED_SERIAL.to_string(), false);
        // Expect no match for second user
        check_duplicate_barcode(&conn, second_user.id, UPDATED_BARCODE.to_string(), false);
        check_duplicate_serial(&conn, second_user.id, UPDATED_SERIAL.to_string(), false);
        check_duplicate_barcode(&conn, second_user.id, UPDATED_BARCODE.to_string(), false);
        check_duplicate_serial(&conn, second_user.id, UPDATED_SERIAL.to_string(), false);

        update(&conn, new_id); // Update Tape
        // Expect match for updated
        check_duplicate_barcode(&conn, first_user.id, TEST_BARCODE.to_string(), false);
        check_duplicate_serial(&conn, first_user.id, TEST_SERIAL.to_string(), false);
        check_duplicate_barcode(&conn, first_user.id, UPDATED_BARCODE.to_string(), true);
        check_duplicate_serial(&conn, first_user.id, UPDATED_SERIAL.to_string(), true);
        // Expect no match for second user
        check_duplicate_barcode(&conn, second_user.id, UPDATED_BARCODE.to_string(), false);
        check_duplicate_serial(&conn, second_user.id, UPDATED_SERIAL.to_string(), false);
        check_duplicate_barcode(&conn, second_user.id, UPDATED_BARCODE.to_string(), false);
        check_duplicate_serial(&conn, second_user.id, UPDATED_SERIAL.to_string(), false);

        delete(&conn, new_id);
        check_duplicate_barcode(&conn, first_user.id, TEST_BARCODE.to_string(), false);
        check_duplicate_serial(&conn, first_user.id, TEST_SERIAL.to_string(), false);
        check_duplicate_barcode(&conn, first_user.id, UPDATED_BARCODE.to_string(), false);
        check_duplicate_serial(&conn, first_user.id, UPDATED_SERIAL.to_string(), false);

        let _ = insert(&conn, second_user.id); // Insert new Tape as second user
        // Expect no match for first user
        check_duplicate_barcode(&conn, first_user.id, TEST_BARCODE.to_string(), false);
        check_duplicate_serial(&conn, first_user.id, TEST_SERIAL.to_string(), false);
        check_duplicate_barcode(&conn, first_user.id, UPDATED_BARCODE.to_string(), false);
        check_duplicate_serial(&conn, first_user.id, UPDATED_SERIAL.to_string(), false);
        // Expect no match for second user
        check_duplicate_barcode(&conn, second_user.id, TEST_BARCODE.to_string(), true);
        check_duplicate_serial(&conn, second_user.id, TEST_SERIAL.to_string(), true);
        check_duplicate_barcode(&conn, second_user.id, UPDATED_BARCODE.to_string(), false);
        check_duplicate_serial(&conn, second_user.id, UPDATED_SERIAL.to_string(), false);
    }
}
