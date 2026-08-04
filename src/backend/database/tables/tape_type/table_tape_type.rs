use std::marker::PhantomData;

use rusqlite::{Connection, params};

use crate::{
    backend::database::tables::table::{
        RecordDelete, RecordFill, RecordInsert, RecordRead, RecordUpdate, TableCreate, TableUpdate,
    },
    shared::models::database::tape_type::model_tape_type::{RecordTapeType, RecordTapeTypeLabel},
};

pub struct TableTapeType<T = RecordTapeType> {
    phantom: PhantomData<T>,
}

impl TableCreate<RecordTapeType> for TableTapeType<RecordTapeType> {
    fn create_table(db: &Connection) -> Result<bool, rusqlite::Error> {
        match db.table_exists(None, "tape_type") {
            std::result::Result::Ok(exist) => {
                if exist {
                    return Ok(false);
                }
            }
            Err(e) => return Err(e),
        }

        db.execute(
            "CREATE TABLE IF NOT EXISTS tape_type (
                id INTEGER PRIMARY KEY,
                generation INTEGER NOT NULL,
                description TEXT NOT NULL,
                id_reg VARCHAR(2),
                id_worm VARCHAR(2),
                native_capacity BIGINT NOT NULL,
                colour_reg VARCHAR(16),
                colour_hp VARCHAR(16),
                colour_worm_reg VARCHAR(16),
                colour_worm_hp VARCHAR(16),
                supports_worm BOOLEAN NOT NULL,
                supports_encryption BOOLEAN NOT NULL,
                supports_ltfs BOOLEAN NOT NULL
            );",
            (),
        )?;

        let bytes_per_gib = 1000 * 1000 * 1000;

        TableTapeType::insert(
            db,
            &RecordTapeType {
                id: 0,
                generation: 1,
                description: "LTO-1".to_string(),
                id_reg: "L1".to_string(),
                id_worm: "".to_string(),
                native_capacity: bytes_per_gib * 100,
                colour_reg: "black".to_string(),
                colour_hp: "blue".to_string(),
                colour_worm_reg: "".to_string(),
                colour_worm_hp: "".to_string(),
                supports_worm: false,
                supports_encryption: false,
                supports_ltfs: false,
            },
        )?;

        TableTapeType::insert(
            db,
            &RecordTapeType {
                id: 0,
                generation: 2,
                description: "LTO-2".to_string(),
                id_reg: "L2".to_string(),
                id_worm: "".to_string(),
                native_capacity: bytes_per_gib * 200,
                colour_reg: "purple".to_string(),
                colour_hp: "red-dark".to_string(),
                colour_worm_reg: "".to_string(),
                colour_worm_hp: "".to_string(),
                supports_worm: false,
                supports_encryption: false,
                supports_ltfs: false,
            },
        )?;

        TableTapeType::insert(
            db,
            &RecordTapeType {
                id: 0,
                generation: 3,
                description: "LTO-3".to_string(),
                id_reg: "L3".to_string(),
                id_worm: "LT".to_string(),
                native_capacity: bytes_per_gib * 400,
                colour_reg: "blue-grey".to_string(),
                colour_hp: "yellow".to_string(),
                colour_worm_reg: "blue-grey".to_string(),
                colour_worm_hp: "yellow".to_string(),
                supports_worm: true,
                supports_encryption: false,
                supports_ltfs: false,
            },
        )?;

        TableTapeType::insert(
            db,
            &RecordTapeType {
                id: 0,
                generation: 4,
                description: "LTO-4".to_string(),
                id_reg: "L4".to_string(),
                id_worm: "LU".to_string(),
                native_capacity: bytes_per_gib * 800,
                colour_reg: "green-dark".to_string(),
                colour_hp: "green".to_string(),
                colour_worm_reg: "green-dark".to_string(),
                colour_worm_hp: "green".to_string(),
                supports_worm: true,
                supports_encryption: true,
                supports_ltfs: false,
            },
        )?;

        TableTapeType::insert(
            db,
            &RecordTapeType {
                id: 0,
                generation: 5,
                description: "LTO-5".to_string(),
                id_reg: "L5".to_string(),
                id_worm: "LV".to_string(),
                native_capacity: bytes_per_gib * 1500,
                colour_reg: "red-dark".to_string(),
                colour_hp: "blue-light".to_string(),
                colour_worm_reg: "red-dark".to_string(),
                colour_worm_hp: "blue-light".to_string(),
                supports_worm: true,
                supports_encryption: true,
                supports_ltfs: true,
            },
        )?;

        TableTapeType::insert(
            db,
            &RecordTapeType {
                id: 0,
                generation: 6,
                description: "LTO-6".to_string(),
                id_reg: "L6".to_string(),
                id_worm: "LW".to_string(),
                native_capacity: bytes_per_gib * 2500,
                colour_reg: "black".to_string(),
                colour_hp: "purple".to_string(),
                colour_worm_reg: "black".to_string(),
                colour_worm_hp: "purple".to_string(),
                supports_worm: true,
                supports_encryption: true,
                supports_ltfs: true,
            },
        )?;

        TableTapeType::insert(
            db,
            &RecordTapeType {
                id: 0,
                generation: 7,
                description: "LTO-7".to_string(),
                id_reg: "L7".to_string(),
                id_worm: "LX".to_string(),
                native_capacity: bytes_per_gib * 6000,
                colour_reg: "purple".to_string(),
                colour_hp: "blue-stale".to_string(),
                colour_worm_reg: "purple".to_string(),
                colour_worm_hp: "blue-stale".to_string(),
                supports_worm: true,
                supports_encryption: true,
                supports_ltfs: true,
            },
        )?;

        TableTapeType::insert(
            db,
            &RecordTapeType {
                id: 0,
                generation: 7,
                description: "LTO-7 Type M8".to_string(),
                id_reg: "M8".to_string(),
                id_worm: "".to_string(),
                native_capacity: bytes_per_gib * 9000,
                colour_reg: "purple".to_string(),
                colour_hp: "blue-stale".to_string(),
                colour_worm_reg: "".to_string(),
                colour_worm_hp: "".to_string(),
                supports_worm: false,
                supports_encryption: true,
                supports_ltfs: true,
            },
        )?;

        TableTapeType::insert(
            db,
            &RecordTapeType {
                id: 0,
                generation: 8,
                description: "LTO-8".to_string(),
                id_reg: "L8".to_string(),
                id_worm: "LY".to_string(),
                native_capacity: bytes_per_gib * 12000,
                colour_reg: "red-dark".to_string(),
                colour_hp: "green".to_string(),
                colour_worm_reg: "red-dark".to_string(),
                colour_worm_hp: "green".to_string(),
                supports_worm: true,
                supports_encryption: true,
                supports_ltfs: true,
            },
        )?;

        TableTapeType::insert(
            db,
            &RecordTapeType {
                id: 0,
                generation: 9,
                description: "LTO-9".to_string(),
                id_reg: "L9".to_string(),
                id_worm: "LZ".to_string(),
                native_capacity: bytes_per_gib * 18000,
                colour_reg: "green-dark".to_string(),
                colour_hp: "blue-light".to_string(),
                colour_worm_reg: "green-dark".to_string(),
                colour_worm_hp: "blue-light".to_string(),
                supports_worm: true,
                supports_encryption: true,
                supports_ltfs: true,
            },
        )?;

        TableTapeType::insert(
            db,
            &RecordTapeType {
                id: 0,
                generation: 10,
                description: "LTO-10 30TB".to_string(),
                id_reg: "LA".to_string(),
                id_worm: "LH".to_string(),
                native_capacity: bytes_per_gib * 30000,
                colour_reg: "black".to_string(),
                colour_hp: "purple".to_string(),
                colour_worm_reg: "black".to_string(),
                colour_worm_hp: "purple".to_string(),
                supports_worm: true,
                supports_encryption: true,
                supports_ltfs: true,
            },
        )?;

        TableTapeType::insert(
            db,
            &RecordTapeType {
                id: 0,
                generation: 10,
                description: "LTO-10 40TB".to_string(),
                id_reg: "PA".to_string(),
                id_worm: "PH".to_string(),
                native_capacity: bytes_per_gib * 40000,
                colour_reg: "black".to_string(),
                colour_hp: "purple".to_string(),
                colour_worm_reg: "".to_string(),
                colour_worm_hp: "".to_string(),
                supports_worm: false,
                supports_encryption: true,
                supports_ltfs: true,
            },
        )?;

        Ok(true)
    }
}

impl TableUpdate<RecordTapeType> for TableTapeType<RecordTapeType> {
    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, rusqlite::Error> {
        Ok(false)
    }
}

impl RecordRead<RecordTapeType> for TableTapeType<RecordTapeType> {
    fn get(db: &Connection, record_id: i64) -> Result<RecordTapeType, rusqlite::Error> {
        db.prepare(
            "SELECT
                    id,
                    generation,
                    description,
                    id_reg,
                    id_worm,
                    native_capacity,
                    colour_reg,
                    colour_hp,
                    colour_worm_reg,
                    colour_worm_hp
                    supports_worm,
                    supports_encryption,
                    supports_ltfs
                FROM tape_type
                WHERE id = ?1",
        )?
        .query_one([record_id], |row| TableTapeType::fill(row, 0))
    }
}

impl RecordInsert<RecordTapeType> for TableTapeType<RecordTapeType> {
    fn insert(db: &Connection, record: &RecordTapeType) -> Result<i64, rusqlite::Error> {
        db.execute(
            "INSERT INTO tape_type (
                    generation,
                    description,
                    id_reg,
                    id_worm,
                    native_capacity,
                    colour_reg,
                    colour_hp,
                    colour_worm_reg,
                    colour_worm_hp,
                    supports_worm,
                    supports_encryption,
                    supports_ltfs)
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
                    ?12
                );",
            params![
                record.generation,
                record.description,
                record.id_reg,
                record.id_worm,
                record.native_capacity,
                record.colour_reg,
                record.colour_hp,
                record.colour_worm_reg,
                record.colour_worm_hp,
                record.supports_worm,
                record.supports_encryption,
                record.supports_ltfs,
            ],
        )?;
        Ok(db.last_insert_rowid())
    }
}

impl RecordUpdate<RecordTapeType> for TableTapeType<RecordTapeType> {
    fn update(db: &Connection, record: &RecordTapeType) -> Result<usize, rusqlite::Error> {
        db.execute(
            "UPDATE tape_type SET
                    generation = ?1,
                    description = ?2,
                    id_reg = ?3,
                    id_worm = ?4,
                    native_capacity = ?5,
                    colour_reg = ?6,
                    colour_hp = ?7,
                    colour_worm_reg = ?8,
                    colour_worm_hp = ?9,
                    supports_worm = ?10,
                    supports_encryption = ?11,
                    supports_ltfs = ?12
                WHERE id = ?13",
            params![
                record.generation,
                record.description,
                record.id_reg,
                record.id_worm,
                record.native_capacity,
                record.colour_reg,
                record.colour_hp,
                record.colour_worm_reg,
                record.colour_worm_hp,
                record.supports_worm,
                record.supports_encryption,
                record.supports_ltfs,
                record.id
            ],
        )
    }
}

impl RecordDelete<RecordTapeType> for TableTapeType<RecordTapeType> {
    fn delete(db: &Connection, record_id: i64) -> Result<usize, rusqlite::Error> {
        db.execute("DELETE FROM tape_type WHERE id = ?1;", params![record_id])
    }
}

impl RecordFill<RecordTapeType> for TableTapeType<RecordTapeType> {
    fn fill(row: &rusqlite::Row<'_>, offset: usize) -> Result<RecordTapeType, rusqlite::Error> {
        Ok(RecordTapeType {
            id: row.get(offset)?,
            generation: row.get(offset + 1)?,
            description: row.get(offset + 2)?,
            id_reg: row.get(offset + 3)?,
            id_worm: row.get(offset + 4)?,
            native_capacity: row.get(offset + 5)?,
            colour_reg: row.get(offset + 6)?,
            colour_hp: row.get(offset + 7)?,
            colour_worm_reg: row.get(offset + 8)?,
            colour_worm_hp: row.get(offset + 9)?,
            supports_worm: row.get(offset + 10)?,
            supports_encryption: row.get(offset + 11)?,
            supports_ltfs: row.get(offset + 12)?,
        })
    }
}

impl TableTapeType<RecordTapeType> {
    pub fn get_all(db: &Connection) -> Result<Vec<RecordTapeType>, rusqlite::Error> {
        db.prepare(
            "SELECT 
                    id,
                    generation,
                    description,
                    id_reg,
                    id_worm,
                    native_capacity,
                    colour_reg,
                    colour_hp,
                    colour_worm_reg,
                    colour_worm_hp,
                    supports_worm,
                    supports_encryption,
                    supports_ltfs
                FROM tape_type
                ORDER BY id",
        )?
        .query_map([], |row| TableTapeType::fill(row, 0))?
        .collect::<Result<Vec<RecordTapeType>, rusqlite::Error>>()
    }
}

impl TableTapeType<RecordTapeTypeLabel> {
    pub fn get_all(db: &Connection) -> Result<Vec<RecordTapeTypeLabel>, rusqlite::Error> {
        let mut results = vec![];
        let mut id = 0;
        let _ = db
            .prepare(
                "SELECT
                    description,
                    id_reg,
                    id_worm
                FROM tape_type
                ORDER BY id",
            )?
            .query_map([], |row| {
                results.push(RecordTapeTypeLabel {
                    id,
                    description: row.get(0)?,
                    designation: row.get(1)?,
                });
                let worm_designation: String = row.get(2)?;
                if !worm_designation.is_empty() {
                    id += 1;
                    results.push(RecordTapeTypeLabel {
                        id,
                        description: format!("{} WORM", row.get::<usize, String>(0)?),
                        designation: worm_designation,
                    });
                }
                id += 1;
                Ok(0) // Dummy, TODO find better way
            })?
            .count();
        results.push(RecordTapeTypeLabel {
            id,
            description: "Cleaning Cartridge".to_string(),
            designation: "CU".to_string(),
        });
        Ok(results)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        backend::database::{
            db::tests::create_test_database,
            tables::{
                table::{RecordUpdate, TableCreate},
                tape_type::table_tape_type::TableTapeType,
            },
        },
        shared::models::database::tape_type::model_tape_type::RecordTapeType,
    };

    pub fn create_table(conn: &rusqlite::Connection) {
        assert!(
            !conn.table_exists(None, "tape_type").unwrap(),
            "New table should be empty"
        );

        let create_result = TableTapeType::create_table(conn);
        if create_result.is_err() {
            println!("--Error create table: {:?}", create_result);
        }
        assert!(create_result.is_ok(), "Failed to create table");
        assert!(
            conn.table_exists(None, "tape_type").unwrap(),
            "create_table() reported Ok but table does not exist"
        );
    }

    fn update(db: &rusqlite::Connection) {
        let all_records_result = TableTapeType::<RecordTapeType>::get_all(db);
        assert!(
            all_records_result.is_ok(),
            "Failed to get all TapeType records"
        );
        let all_records = all_records_result.unwrap();
        assert!(!all_records.is_empty(), "Default not populated");
        let test_index = all_records.len() / 2;
        let original_record: RecordTapeType = all_records.get(test_index).unwrap().clone();

        let new_name = "abc".to_string();
        let mut update_record: RecordTapeType = original_record.clone();
        update_record.description = new_name;
        assert!(
            TableTapeType::update(db, &update_record).is_ok(),
            "Failed to update record"
        );

        let all_records_updated = TableTapeType::<RecordTapeType>::get_all(db).unwrap();
        assert_eq!(update_record, *all_records_updated.get(test_index).unwrap());
    }

    #[test]
    fn suite() {
        let conn = create_test_database();
        create_table(&conn);
        update(&conn);
    }
}
