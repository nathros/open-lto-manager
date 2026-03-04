use rusqlite::{Connection, Row};

pub trait Table<T, J> {
    fn create_table(db: &Connection) -> Result<bool, rusqlite::Error>;
    fn update_table(db: &Connection, current_version: i64) -> Result<bool, rusqlite::Error>;

    fn get(db: &Connection, record_id: i64) -> Result<T, rusqlite::Error>;
    #[allow(dead_code)] // FIXME
    fn get_join(db: &Connection, record_id: i64) -> Result<J, rusqlite::Error>;

    fn insert_record(db: &Connection, record: &T) -> Result<i64, rusqlite::Error>;
    fn insert_batch(db: &Connection, records: &[T]) -> Result<usize, rusqlite::Error>;

    fn update_record(db: &Connection, record: &T) -> Result<usize, rusqlite::Error>;
    fn delete_record(db: &Connection, record_id: i64) -> Result<usize, rusqlite::Error>;

    fn fill(row: &Row<'_>, offset: usize) -> Result<T, rusqlite::Error>;
}
