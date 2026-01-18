use std::io::ErrorKind;

#[cfg(feature = "server")]
use dioxus::prelude::{error, info};

#[cfg(feature = "server")]
use crate::backend::database::tables::table_tape::TableTape;
use crate::backend::database::tables::{
    table::Table, table_manufacturer::TableManufacturer, table_tape_type::TableTapeType,
};

//static DB_VERSION_INIT: isize = 0;
static DB_VERSION_LATEST: isize = 0;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: std::sync::LazyLock<rusqlite::Connection> = std::sync::LazyLock::new(|| {
        match std::fs::create_dir("database") {
            Ok(_) => {},
            Err(err) => {
                if err.kind() != ErrorKind::AlreadyExists {
                    // log::error!("Failed to create dir {e}"); FIXME
                }
            },
        }
        let conn = rusqlite::Connection::open("database/database.db").expect("Failed to open database");
        let current_database_version: isize = 0;

        // FIXME handle errors
         if let Ok(_) = TableManufacturer::create_table(&conn) {
            info!("TableManufacturer created");
            let _ = TableManufacturer::update_table(current_database_version, DB_VERSION_LATEST);
        } else {
            error!("TableManufacturer not created");
        }

        if let Ok(_) = TableTapeType::create_table(&conn) {
            info!("TableTapeType created");
            let _ = TableTapeType::update_table(current_database_version, DB_VERSION_LATEST);
        } else {
            error!("TableTapeType not created");
        }

        if let Ok(_) = TableTape::create_table(&conn) { // This table as FKs from TableTapeType and TableManufacturer
            info!("TableTape created");
            let _ = TableTape::update_table(current_database_version, DB_VERSION_LATEST);
        } else {
            error!("TableTape not created");
        }

        conn
    });
}
