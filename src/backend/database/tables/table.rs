use rusqlite::{Connection, Error, Row};

pub trait Table<T, J> {
    fn create_table(db: &Connection) -> Result<bool, Error>;
    fn update_table(db: &Connection, current_version: i64) -> Result<bool, Error>;

    fn get(db: &Connection, record_id: i64) -> Result<T, Error>;
    fn get_join(db: &Connection, record_id: i64) -> Result<J, Error>;

    fn insert_record(db: &Connection, record: &T) -> Result<usize, Error>;
    fn update_record(db: &Connection, record: &T) -> Result<usize, Error>;
    fn delete_record(db: &Connection, record_id: i64) -> Result<usize, Error>;

    fn fill(row: &Row<'_>, offset: usize) -> Result<T, Error>;
}
