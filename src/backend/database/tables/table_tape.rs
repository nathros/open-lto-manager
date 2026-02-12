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
                    tape_type,_id
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
                FROM tape_type
                WHERE id = ?1",
        )?
        .query_one([record_id], |row| TableTape::fill(row, 0))
    }

    fn get_join(_db: &Connection, _record_id: i64) -> Result<RecordTapeJoin, Error> {
        todo!()
    }

    fn insert_record(db: &Connection, record: &RecordTape) -> Result<usize, Error> {
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
        )
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
                    last_used = ?13,
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
