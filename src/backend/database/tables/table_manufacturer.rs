use rusqlite::{params, Connection, Error, Row};

use crate::backend::database::{
    models::model_manufacturer::RecordManufacturer, tables::table::Table,
};

pub struct TableManufacturer {}

impl Table<RecordManufacturer, RecordManufacturer> for TableManufacturer {
    fn create_table(db: &Connection) -> Result<bool, Error> {
        match db.table_exists(None, "manufacturer") {
            std::result::Result::Ok(exist) => {
                if exist == true {
                    return Ok(false);
                }
            }
            Err(e) => return Err(e),
        }

        if let Err(e) = db.execute(
            "CREATE TABLE IF NOT EXISTS manufacturer (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );",
            (),
        ) {
            return Err(e); // Failed to create table
        }

        let manufacturers = vec![
            "Other",
            "Dell",
            "Fujifilm",
            "HP",
            "IBM",
            "Imation",
            "Maxell",
            "Overland Tandberg",
            "Quantum",
            "SONY",
            "Spectra",
            "TDK",
        ];

        for m_name in manufacturers.iter() {
            if let Err(e) = TableManufacturer::insert_record(
                db,
                &RecordManufacturer {
                    id: 0,
                    name: m_name.to_string(),
                },
            ) {
                return Err(e);
            }
        }

        return Ok(true);
    }

    fn update_table(_db: &Connection, _current_version: i64) -> Result<bool, Error> {
        Ok(false)
    }

    fn get(db: &Connection, record_id: i64) -> Result<RecordManufacturer, Error> {
        db.prepare(
            "SELECT id, name,
                    FROM manufacturer
                    WHERE id = ?1",
        )?
        .query_one([record_id], |row| TableManufacturer::fill(row, 0))
    }

    fn get_join(db: &Connection, record_id: i64) -> Result<RecordManufacturer, Error> {
        TableManufacturer::get(db, record_id)
    }

    fn insert_record(db: &Connection, record: &RecordManufacturer) -> Result<usize, Error> {
        db.execute(
            "INSERT INTO manufacturer (name) VALUES (?1)",
            params![record.name],
        )
    }

    fn update_record(db: &Connection, record: &RecordManufacturer) -> Result<usize, Error> {
        db.execute(
            "UPDATE manufacturer SET name = ?1
                WHERE id = ?2;",
            params![record.name, record.id],
        )
    }

    fn delete_record(db: &Connection, record_id: i64) -> Result<usize, Error> {
        db.execute(
            "DELETE FROM manufacturer WHERE id = ?1;",
            params![record_id],
        )
    }

    fn fill(row: &Row<'_>, _offset: usize) -> Result<RecordManufacturer, Error> {
        Ok(RecordManufacturer {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }
}

impl TableManufacturer {
    pub fn get_all(db: &Connection) -> Result<Vec<RecordManufacturer>, rusqlite::Error> {
        db.prepare(
            "SELECT * FROM manufacturer ORDER BY
                    CASE id
                        WHEN 1 THEN 2
                    END,
                    name", // Order by name then "Other" [id=1] to be last
        )?
        .query_map([], |row| TableManufacturer::fill(row, 0))?
        .collect::<Result<Vec<RecordManufacturer>, rusqlite::Error>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::database::{
        models::model_manufacturer::RecordManufacturer,
        tables::{table::Table, table_manufacturer::TableManufacturer},
    };

    fn create() -> rusqlite::Connection {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        assert_eq!(
            conn.table_exists(None, "manufacturer").unwrap(),
            false,
            "New table should be empty"
        );
        assert!(
            TableManufacturer::create_table(&conn).is_ok(),
            "Failed to create table"
        );
        assert_eq!(
            conn.table_exists(None, "manufacturer").unwrap(),
            true,
            "create_table() reported Ok but table does not exist"
        );
        return conn;
    }

    fn update(db: &rusqlite::Connection) {
        let all_records_result = TableManufacturer::get_all(&db);
        assert!(
            all_records_result.is_ok(),
            "Failed to get all Manufacturer records"
        );
        let all_records = all_records_result.unwrap();
        assert!(all_records.len() > 0, "Default not populated");
        assert_eq!(
            all_records.last().unwrap().name,
            "Other",
            "Should end with 'Other' manufacturer"
        );
        let test_index = all_records.len() / 2;
        let original_record: RecordManufacturer = all_records.get(test_index).unwrap().clone();

        let mut new_name = original_record.name.clone();
        new_name.push_str("abc"); // Preserve ordering
        let mut update_record: RecordManufacturer = original_record.clone();
        update_record.name = new_name;
        assert!(
            TableManufacturer::update_record(&db, &update_record).is_ok(),
            "Failed to update record"
        );

        let all_records_updated = TableManufacturer::get_all(&db).unwrap();
        assert_eq!(update_record, *all_records_updated.get(test_index).unwrap());
    }

    fn insert(db: &rusqlite::Connection) {
        let all_records_current = TableManufacturer::get_all(&db).unwrap();
        let new_manufacturer_name = "NewName".to_string();
        assert_eq!(
            all_records_current
                .iter()
                .find(|&m| m.name == new_manufacturer_name),
            None
        );
        TableManufacturer::insert_record(
            &db,
            &RecordManufacturer {
                id: 0,
                name: new_manufacturer_name.clone(),
            },
        )
        .unwrap();

        let all_records_updated = TableManufacturer::get_all(&db).unwrap();
        assert_eq!(
            all_records_updated
                .iter()
                .find(|&m| m.name == new_manufacturer_name)
                .is_some(),
            true
        );

        assert!(
            all_records_updated.last().unwrap().name == "Other",
            "Other not at end of list after insert"
        );
    }

    #[test]
    fn suite() {
        let db = create();
        update(&db);
        insert(&db);
    }
}
