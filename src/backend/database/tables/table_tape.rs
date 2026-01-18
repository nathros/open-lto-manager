use rusqlite::{params, Connection, Error};

use crate::backend::database::{
    models::model_tape::RecordTape, models::model_tape::RecordTapeJoin, tables::table::Table,
};

pub struct TableTape {}

impl Table<RecordTape, RecordTapeJoin> for TableTape {
    fn create_table(db: &Connection) -> Result<(), Error> {
        match db.table_exists(None, "tape") {
            std::result::Result::Ok(exist) => {
                if exist == true {
                    return Ok(());
                }
            }
            Err(e) => return Err(e),
        }

        if let Err(e) = db.execute(
            "CREATE TABLE IF NOT EXISTS tape (
                id INTEGER PRIMARY KEY,
                manufacturer_id INTEGER NOT NULL,
                tape_type_id INTEGER NOT NULL,
                barcode VARCHAR(8) UNIQUE,
                serial TEXT UNIQUE,
                format INTEGER NOT NULL,
                worm BOOLEAN NOT NULL,
                encrypted BOOLEAN NOT NULL,
                compressed BOOLEAN NOT NULL,
                used_space INTEGER NOT NULL,
                created BIGINT NOT NULL,
                last_used BIGINT NOT NULL,
                FOREIGN KEY(manufacturer_id) REFERENCES manufacturer(id),
                FOREIGN KEY(tape_type_id) REFERENCES tape_type(id)
            );",
            (),
        ) {
            return Err(e); // Failed to create table
        }
        return Ok(());
    }

    fn update_table(_current_version: isize, _latest_version: isize) -> Result<(), Error> {
        Ok(())
    }

    fn get(_db: &Connection, _record_id: i64) -> Result<RecordTape, Error> {
        todo!()
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
                    encrypted,
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
                    ?11);",
            params![
                record.manufacturer_id,
                record.tape_type_id,
                record.barcode,
                record.serial,
                record.format,
                record.worm,
                record.encrypted,
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
                    encrypted = ?7,
                    compressed = ?8,
                    used_space = ?9,
                    created = ?10,
                    last_used = ?11,
                WHERE id = ?12",
            params![
                record.manufacturer_id,
                record.tape_type_id,
                record.barcode,
                record.serial,
                record.format,
                record.worm,
                record.encrypted,
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

    fn fill(_row: &rusqlite::Row<'_>, _offset: usize) -> Result<RecordTape, Error> {
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
