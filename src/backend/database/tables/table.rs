use rusqlite::{Connection, Row};

pub trait TableCreate<T> {
    fn create_table(db: &Connection) -> Result<bool, rusqlite::Error>;
}

pub trait TableUpdate<T> {
    fn update_table(db: &Connection, current_version: i64) -> Result<bool, rusqlite::Error>;
}

pub trait RecordRead<T> {
    fn get(db: &Connection, record_id: i64) -> Result<T, rusqlite::Error>;
}

pub trait RecordInsert<T> {
    fn insert(db: &Connection, record: &T) -> Result<i64, rusqlite::Error>;
}

pub trait RecordInsertBatch<T> {
    fn insert_batch(db: &Connection, records: &[T]) -> Result<usize, rusqlite::Error>;
}
pub trait RecordUpdate<T> {
    fn update(db: &Connection, record: &T) -> Result<usize, rusqlite::Error>;
}

pub trait RecordDelete<T> {
    fn delete(db: &Connection, record_id: i64) -> Result<usize, rusqlite::Error>;
}

pub trait TableClear<T> {
    fn clear_table(db: &Connection) -> Result<usize, rusqlite::Error>;
}

pub trait RecordFill<T> {
    fn fill(row: &Row<'_>, offset: usize) -> Result<T, rusqlite::Error>;
}
