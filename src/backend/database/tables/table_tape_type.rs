use dioxus::prelude::info;
use rusqlite::{params, Connection, Error};

use crate::backend::database::{models::model_tape_type::RecordTapeType, tables::table::Table};

pub struct TableTapeType {}

impl Table<RecordTapeType, RecordTapeType> for TableTapeType {
    fn create_table(db: &Connection) -> Result<bool, Error> {
        match db.table_exists(None, "tape_type") {
            std::result::Result::Ok(exist) => {
                if exist == true {
                    return Ok(false);
                }
            }
            Err(e) => return Err(e),
        }

        if let Err(e) = db.execute(
            "CREATE TABLE IF NOT EXISTS tape_type (
                id INTEGER PRIMARY KEY,
                generation TEXT NOT NULL,
                id_reg VARCHAR(2),
                id_worm VARCHAR(2),
                native_capacity BIGINT NOT NULL,
                colour_reg VARCHAR(16),
                colour_hp VARCHAR(16),
                colour_worm_reg VARCHAR(16),
                colour_worm_hp VARCHAR(16)
            );",
            (),
        ) {
            return Err(e); // Failed to create table
        }

        let bytes_per_gib = 1000 * 1000 * 1000;

        if let Err(e) = TableTapeType::insert_record(
            db,
            &RecordTapeType {
                id: 0,
                generation: "LTO-1".to_string(),
                id_reg: "L1".to_string(),
                id_worm: "".to_string(),
                native_capacity: bytes_per_gib * 100,
                colour_reg: "black".to_string(),
                colour_hp: "blue".to_string(),
                colour_worm_reg: "".to_string(),
                colour_worm_hp: "".to_string(),
            },
        ) {
            info!("{e}");
            return Err(e);
        }

        if let Err(e) = TableTapeType::insert_record(
            db,
            &RecordTapeType {
                id: 0,
                generation: "LTO-2".to_string(),
                id_reg: "L2".to_string(),
                id_worm: "".to_string(),
                native_capacity: bytes_per_gib * 200,
                colour_reg: "purple".to_string(),
                colour_hp: "red-dark".to_string(),
                colour_worm_reg: "".to_string(),
                colour_worm_hp: "".to_string(),
            },
        ) {
            return Err(e);
        }

        if let Err(e) = TableTapeType::insert_record(
            db,
            &RecordTapeType {
                id: 0,
                generation: "LTO-3".to_string(),
                id_reg: "L3".to_string(),
                id_worm: "LT".to_string(),
                native_capacity: bytes_per_gib * 400,
                colour_reg: "blue-grey".to_string(),
                colour_hp: "yellow".to_string(),
                colour_worm_reg: "blue-grey".to_string(),
                colour_worm_hp: "yellow".to_string(),
            },
        ) {
            return Err(e);
        }

        if let Err(e) = TableTapeType::insert_record(
            db,
            &RecordTapeType {
                id: 0,
                generation: "LTO-4".to_string(),
                id_reg: "L4".to_string(),
                id_worm: "LU".to_string(),
                native_capacity: bytes_per_gib * 800,
                colour_reg: "green-dark".to_string(),
                colour_hp: "green".to_string(),
                colour_worm_reg: "green-dark".to_string(),
                colour_worm_hp: "green".to_string(),
            },
        ) {
            return Err(e);
        }

        if let Err(e) = TableTapeType::insert_record(
            db,
            &RecordTapeType {
                id: 0,
                generation: "LTO-5".to_string(),
                id_reg: "L5".to_string(),
                id_worm: "LV".to_string(),
                native_capacity: bytes_per_gib * 1500,
                colour_reg: "red-dark".to_string(),
                colour_hp: "blue-light".to_string(),
                colour_worm_reg: "red-dark".to_string(),
                colour_worm_hp: "blue-light".to_string(),
            },
        ) {
            return Err(e);
        }

        if let Err(e) = TableTapeType::insert_record(
            db,
            &RecordTapeType {
                id: 0,
                generation: "LTO-6".to_string(),
                id_reg: "L6".to_string(),
                id_worm: "LW".to_string(),
                native_capacity: bytes_per_gib * 2500,
                colour_reg: "black".to_string(),
                colour_hp: "purple".to_string(),
                colour_worm_reg: "black".to_string(),
                colour_worm_hp: "purple".to_string(),
            },
        ) {
            return Err(e);
        }

        if let Err(e) = TableTapeType::insert_record(
            db,
            &RecordTapeType {
                id: 0,
                generation: "LTO-7".to_string(),
                id_reg: "L7".to_string(),
                id_worm: "LX".to_string(),
                native_capacity: bytes_per_gib * 6000,
                colour_reg: "purple".to_string(),
                colour_hp: "blue-stale".to_string(),
                colour_worm_reg: "purple".to_string(),
                colour_worm_hp: "blue-stale".to_string(),
            },
        ) {
            return Err(e);
        }

        if let Err(e) = TableTapeType::insert_record(
            db,
            &RecordTapeType {
                id: 0,
                generation: "LTO-7 Type M8".to_string(),
                id_reg: "M8".to_string(),
                id_worm: "".to_string(),
                native_capacity: bytes_per_gib * 9000,
                colour_reg: "purple".to_string(),
                colour_hp: "blue-stale".to_string(),
                colour_worm_reg: "".to_string(),
                colour_worm_hp: "".to_string(),
            },
        ) {
            return Err(e);
        }

        if let Err(e) = TableTapeType::insert_record(
            db,
            &RecordTapeType {
                id: 0,
                generation: "LTO-8".to_string(),
                id_reg: "L8".to_string(),
                id_worm: "LY".to_string(),
                native_capacity: bytes_per_gib * 12000,
                colour_reg: "red-dark".to_string(),
                colour_hp: "green".to_string(),
                colour_worm_reg: "red-dark".to_string(),
                colour_worm_hp: "green".to_string(),
            },
        ) {
            return Err(e);
        }

        if let Err(e) = TableTapeType::insert_record(
            db,
            &RecordTapeType {
                id: 0,
                generation: "LTO-9".to_string(),
                id_reg: "L9".to_string(),
                id_worm: "LZ".to_string(),
                native_capacity: bytes_per_gib * 18000,
                colour_reg: "green-dark".to_string(),
                colour_hp: "blue-light".to_string(),
                colour_worm_reg: "green-dark".to_string(),
                colour_worm_hp: "blue-light".to_string(),
            },
        ) {
            return Err(e);
        }

        if let Err(e) = TableTapeType::insert_record(
            db,
            &RecordTapeType {
                id: 0,
                generation: "LTO-10 30TB".to_string(),
                id_reg: "LA".to_string(),
                id_worm: "LH".to_string(),
                native_capacity: bytes_per_gib * 30000,
                colour_reg: "black".to_string(),
                colour_hp: "purple".to_string(),
                colour_worm_reg: "black".to_string(),
                colour_worm_hp: "purple".to_string(),
            },
        ) {
            return Err(e);
        }

        if let Err(e) = TableTapeType::insert_record(
            db,
            &RecordTapeType {
                id: 0,
                generation: "LTO-10 40TB".to_string(),
                id_reg: "PA".to_string(),
                id_worm: "PH".to_string(),
                native_capacity: bytes_per_gib * 40000,
                colour_reg: "black".to_string(),
                colour_hp: "purple".to_string(),
                colour_worm_reg: "".to_string(),
                colour_worm_hp: "".to_string(),
            },
        ) {
            return Err(e);
        }

        return Ok(true);
    }

    fn update_table(
        _db: &Connection,
        _current_version: isize,
        _latest_version: isize,
    ) -> Result<bool, Error> {
        Ok(false)
    }

    fn get(db: &Connection, record_id: i64) -> Result<RecordTapeType, Error> {
        db.prepare("SELECT * FROM manufacturer WHERE id = ?1")?
            .query_one([record_id], |row| TableTapeType::fill(row, 0))
    }

    fn get_join(db: &Connection, record_id: i64) -> Result<RecordTapeType, Error> {
        TableTapeType::get(db, record_id)
    }

    fn insert_record(db: &Connection, record: &RecordTapeType) -> Result<usize, Error> {
        db.execute(
            "INSERT INTO tape_type (
                    generation,
                    id_reg,
                    id_worm,
                    native_capacity,
                    colour_reg,
                    colour_hp,
                    colour_worm_reg,
                    colour_worm_hp)
                VALUES (
                    ?1,
                    ?2,
                    ?3,
                    ?4,
                    ?5,
                    ?6,
                    ?7,
                    ?8
                );",
            params![
                record.generation,
                record.id_reg,
                record.id_worm,
                record.native_capacity,
                record.colour_reg,
                record.colour_hp,
                record.colour_worm_reg,
                record.colour_worm_hp
            ],
        )
    }

    fn update_record(db: &Connection, record: &RecordTapeType) -> Result<usize, Error> {
        db.execute(
            "UPDATE tape_type SET 
                    generation = ?1,
                    id_reg = ?2,
                    id_worm = ?3,
                    native_capacity = ?,4
                    colour_reg = ?5,
                    colour_hp = ?6,
                    colour_worm_reg = ?7,
                    colour_worm_hp = ?8
                WHERE id = ?9",
            params![
                record.generation,
                record.id_reg,
                record.id_worm,
                record.native_capacity,
                record.colour_reg,
                record.colour_hp,
                record.colour_worm_reg,
                record.colour_worm_hp,
                record.id
            ],
        )
    }

    fn delete_record(db: &Connection, record_id: i64) -> Result<usize, Error> {
        db.execute("DELETE FROM tape_type WHERE id = ?1;", params![record_id])
    }

    fn fill(row: &rusqlite::Row<'_>, _offset: usize) -> Result<RecordTapeType, Error> {
        Ok(RecordTapeType {
            id: row.get(0)?,
            generation: row.get(1)?,
            id_reg: row.get(2)?,
            id_worm: row.get(3)?,
            native_capacity: row.get(4)?,
            colour_reg: row.get(5)?,
            colour_hp: row.get(6)?,
            colour_worm_reg: row.get(7)?,
            colour_worm_hp: row.get(8)?,
        })
    }
}

impl TableTapeType {
    pub fn get_all(db: &Connection) -> Result<Vec<RecordTapeType>, rusqlite::Error> {
        db.prepare(
            "SELECT 
                    id,
                    generation,
                    id_reg,
                    id_worm,
                    native_capacity,
                    colour_reg,
                    colour_hp,
                    colour_worm_reg,
                    colour_worm_hp
                FROM tape_type
                ORDER BY id",
        )?
        .query_map([], |row| {
            Ok(RecordTapeType {
                id: row.get(0)?,
                generation: row.get(1)?,
                id_reg: row.get(2)?,
                id_worm: row.get(3)?,
                native_capacity: row.get(4)?,
                colour_reg: row.get(5)?,
                colour_hp: row.get(6)?,
                colour_worm_reg: row.get(7)?,
                colour_worm_hp: row.get(8)?,
            })
        })?
        .collect::<Result<Vec<RecordTapeType>, rusqlite::Error>>()
    }
}
