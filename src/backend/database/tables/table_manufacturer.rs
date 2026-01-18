use rusqlite::{params, Connection, Error, Row};

use crate::backend::database::{
    models::model_manufacturer::RecordManufacturer, tables::table::Table,
};

pub struct TableManufacturer {}

impl Table<RecordManufacturer, RecordManufacturer> for TableManufacturer {
    fn create_table(db: &Connection) -> Result<(), Error> {
        match db.table_exists(None, "manufacturer") {
            std::result::Result::Ok(exist) => {
                if exist == true {
                    return Ok(());
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

        return Ok(());
    }

    fn update_table(_current_version: isize, _latest_version: isize) -> Result<(), Error> {
        Ok(())
    }

    fn get(db: &Connection, record_id: i64) -> Result<RecordManufacturer, Error> {
        db.prepare("SELECT * FROM manufacturer WHERE id = ?1")?
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
            "Failded to create table"
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
            "Failed to get all manufacturer records"
        );
        let all_records = all_records_result.unwrap();
        assert!(all_records.len() > 0, "Default not populated");
        assert_eq!(
            all_records.last().unwrap().name,
            "Other",
            "Should end with 'Other' manufacturer"
        );
        let test_index = all_records.len() / 2;
        let orginal_record: RecordManufacturer = all_records.get(test_index).unwrap().clone();

        let new_name = "abc".to_string();
        let mut update_record: RecordManufacturer = orginal_record.clone();
        update_record.name = new_name;
        assert!(
            TableManufacturer::update_record(&db, &update_record).is_ok(),
            "Failed to update record"
        );
    }

    //fn insert(db: &rusqlite::Connection) {}

    #[test]
    fn suite() {
        let db = create();
        update(&db);
    }
}
