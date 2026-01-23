#[cfg(feature = "server")]
use std::io::ErrorKind;
use std::sync::Mutex;

use dioxus::prelude::info;
use rusqlite::{Connection, Error};

use crate::backend::{database::tables::table_tape::TableTape, env::get_database_path};
use crate::backend::{
    database::{
        models::model_version::RecordVersion,
        tables::{
            table::Table, table_manufacturer::TableManufacturer, table_tape_type::TableTapeType,
            table_user::TableUser, table_version::TableVersion,
        },
    },
    env::get_database_file,
};

static DB_VERSION_LATEST: i64 = 0;

fn database_init(conn: rusqlite::Connection) -> Result<rusqlite::Connection, String> {
    let current_database_version: i64;

    match TableVersion::create_table(&conn) {
        Ok(created) => {
            if created {
                match TableVersion::insert_record(
                    &conn,
                    &RecordVersion {
                        version_number: DB_VERSION_LATEST,
                    },
                ) {
                    Ok(_) => current_database_version = DB_VERSION_LATEST,
                    Err(e) => return Err(format!("Failed to set table version {}", e)),
                }
            } else {
                match TableVersion::get(&conn, 1) {
                    Ok(v) => current_database_version = v.version_number,
                    Err(e) => return Err(format!("Failed to get table version {}", e)),
                };
            }
        }
        Err(e) => return Err(format!("TableVersion creation error {}", e)),
    }
    info!(
        "Database version: {}, latest is : {}",
        current_database_version, DB_VERSION_LATEST
    );

    let tables: Vec<(
        &str,
        &dyn Fn(&Connection) -> Result<bool, Error>,
        &dyn Fn(&Connection, i64) -> Result<bool, Error>,
    )> = vec![
        (
            "TableUser",
            &TableUser::create_table,
            &TableUser::update_table,
        ),
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

    // Create tables and update them if needed
    for (table_name, create_fn, update_fn) in tables.iter() {
        match create_fn(&conn) {
            Ok(created) => {
                info!(
                    "{} {}",
                    table_name,
                    if created { "created" } else { "already exists" }
                );
                match update_fn(&conn, current_database_version) {
                    Ok(updated) => {
                        if updated {
                            info!("{} updated to v{}", table_name, DB_VERSION_LATEST);
                        }
                    }
                    Err(e) => return Err(format!("{} update error {}", table_name, e)),
                }
            }
            Err(e) => return Err(format!("{} creation error {}", table_name, e)),
        }
    }

    if current_database_version != DB_VERSION_LATEST {
        match TableVersion::update_record(
            &conn,
            &RecordVersion {
                version_number: DB_VERSION_LATEST,
            },
        ) {
            Ok(_) => info!(
                "Upgraded table from {} to {}",
                current_database_version, DB_VERSION_LATEST
            ),
            Err(e) => {
                return Err(format!(
                    "Failed table upgrade from {} to {}, {}",
                    current_database_version, DB_VERSION_LATEST, e
                ))
            }
        }
    }

    Ok(conn)
}

pub fn create_database() -> Result<rusqlite::Connection, String> {
    let mut db_path = get_database_path();

    match std::fs::create_dir_all(&db_path) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() != ErrorKind::AlreadyExists {
                return Err(format!("Failed to create database dir: {}", e));
            }
        }
    }
    db_path = get_database_file();
    static FIRST_RUN: Mutex<bool> = Mutex::new(true);

    match rusqlite::Connection::open(&db_path) {
        Ok(conn) => match FIRST_RUN.try_lock() {
            Ok(mut guard) => {
                if *guard == true {
                    *guard = false;
                    info!("Open database at path: {}", db_path);
                    return database_init(conn);
                } else {
                    return Ok(conn);
                }
            }
            Err(e) => return Err(format!("Failed to get database init lock: {}", e)),
        },
        Err(e) => return Err(format!("Failed to open database: {}", e)),
    }
}

thread_local! {
    pub static DB: std::sync::LazyLock<rusqlite::Connection> = std::sync::LazyLock::new(|| {
        create_database().unwrap() // In separate function as rustfmt does not work inside this closure
    });
}
