use crate::backend::database::{
    db::{DB, tests::create_test_database},
    tables::{
        file::{
            table_file::TableFile,
            tests::test_data::{get_file_test_data_1, get_test_tape},
        },
        table::RecordInsert,
        tape::{self, table_tape::TableTape},
    },
};

fn create_table() -> rusqlite::Connection {
    let conn = create_test_database();
    // TableFile depends on TableManufacturer, TableTapeType and TableTape, so these must be created first
    tape::table_tape::tests::create_table(&conn); // This creates TableManufacturer and TableTapeType
    conn
}

fn insert_test_data(db: &rusqlite::Connection) {
    let tape_1 = get_test_tape();
    let tape_id_1 = TableTape::insert(db, &tape_1).unwrap();

    let files = get_file_test_data_1(tape_id_1);
    for file in files {
        let _ = TableFile::insert(db, &file).unwrap();
    }
}

#[test]
fn create() {
    let _db = create_table();
}

#[test]
fn insert() {
    let _db = create_table();
}

#[test]
fn advanced_query() {
    let db = create_table();
    insert_test_data(&db);
}

#[ignore = "Used to add test data to database"]
#[test]
fn add_test_data() {
    DB.with(|db| {
        insert_test_data(db);
    });
}
