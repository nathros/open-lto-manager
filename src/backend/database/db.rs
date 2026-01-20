#[cfg(feature = "server")]
use std::io::ErrorKind;

#[cfg(feature = "server")]
use dioxus::prelude::{error, info};

use crate::backend::database::tables::{
    table::Table, table_manufacturer::TableManufacturer, table_tape_type::TableTapeType,
};
#[cfg(feature = "server")]
use crate::backend::{database::tables::table_tape::TableTape, env::get_database_path};

//static DB_VERSION_INIT: isize = 0;
static DB_VERSION_LATEST: isize = 0;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: std::sync::LazyLock<rusqlite::Connection> = std::sync::LazyLock::new(|| {
        let mut db_path = get_database_path();
        info!("Database path: {}", db_path);

        match std::fs::create_dir_all(&db_path) {
            Ok(_) => {},
            Err(err) => {
                if err.kind() != ErrorKind::AlreadyExists {
                    // log::error!("Failed to create dir {e}"); FIXME
                    error!("Failed to create database dir: {}", err);
                }
            },
        }
        db_path.push_str("/database.db");
        let conn = rusqlite::Connection::open(db_path).expect("Failed to open database");
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
