#[cfg(feature = "server")]
use std::io::ErrorKind;

#[cfg(feature = "server")]
use dioxus::prelude::{error, info};
#[cfg(feature = "server")]
use rusqlite::{Connection, Error};

use crate::backend::database::tables::{
    table::Table, table_manufacturer::TableManufacturer, table_tape_type::TableTapeType,
};
#[cfg(feature = "server")]
use crate::backend::{database::tables::table_tape::TableTape, env::get_database_path};

//static DB_VERSION_INIT: isize = 0;
static DB_VERSION_LATEST: isize = 0;

fn create_database() -> rusqlite::Connection {
    let mut db_path = get_database_path();
    info!("Database path: {}", db_path);

    match std::fs::create_dir_all(&db_path) {
        Ok(_) => {}
        Err(err) => {
            if err.kind() != ErrorKind::AlreadyExists {
                // log::error!("Failed to create dir {e}"); FIXME
                error!("Failed to create database dir: {}", err);
            }
        }
    }
    db_path.push_str("/database.db");
    let conn = rusqlite::Connection::open(db_path).expect("Failed to open database");
    let current_database_version: isize = 0;

    let tables: Vec<(
        &str,
        &dyn Fn(&Connection) -> Result<bool, Error>,
        &dyn Fn(&Connection, isize, isize) -> Result<bool, Error>,
    )> = vec![
        (
            "TableManufacturer",
            &TableManufacturer::create_table,
            &TableManufacturer::update_table,
        ),
        (
            "TableTapeType",
            &TableTapeType::create_table,
            &TableTapeType::update_table,
        ),
        (
            "TableTape",
            &TableTape::create_table,
            &TableTape::update_table,
        ),
    ];

    // FIXME handle errors poison, this is not propagated to UI
    // Create tables and update them if needed
    for (table_name, create_fn, update_fn) in tables.iter() {
        match create_fn(&conn) {
            Ok(created) => {
                info!(
                    "{} {}",
                    table_name,
                    if created { "created" } else { "already exists" }
                );
                match update_fn(&conn, current_database_version, DB_VERSION_LATEST) {
                    Ok(updated) => {
                        if updated {
                            info!("{} updated to v{}", table_name, DB_VERSION_LATEST);
                        }
                    }
                    Err(e) => error!("{} update error {}", table_name, e),
                }
            }
            Err(e) => error!("{} creation error {}", table_name, e),
        }
    }

    conn
}

#[cfg(feature = "server")]
thread_local! {
    pub static DB: std::sync::LazyLock<rusqlite::Connection> = std::sync::LazyLock::new(|| {
        create_database() // In separate function as rustfmt does not work inside this closure
    });
}
