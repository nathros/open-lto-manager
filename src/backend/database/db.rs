use dioxus::prelude::info;
use rusqlite::{Connection, Error, params};
use std::{
    io::ErrorKind,
    sync::{LazyLock, Mutex},
};

use crate::{
    backend::{
        database::tables::{
            file::table_file::TableFile,
            job::table_job::TableJob,
            job_metadata::table_job_metadata::TableJobMetadata,
            label_preset::table_label_preset::TableLabelPreset,
            manufacturer::table_manufacturer::TableManufacturer,
            setting::table_setting::TableSetting,
            table::{RecordInsert, RecordUpdate, TableCreate, TableUpdate},
            tape::table_tape::TableTape,
            tape_type::table_tape_type::TableTapeType,
            user::table_user::TableUser,
        },
        env::{get_database_file, get_database_path},
    },
    shared::models::database::setting::{
        model_setting::{RecordMisc, SettingsKey},
        types_setting::SettingTableVersion,
    },
};

static DB_VERSION_LATEST: i64 = 0;

fn database_init(conn: rusqlite::Connection) -> Result<rusqlite::Connection, String> {
    let current_database_version: i64;

    match TableSetting::create_table(&conn) {
        Ok(created) => {
            if created {
                match TableSetting::<RecordMisc<SettingTableVersion>>::insert(
                    &conn,
                    &RecordMisc::<SettingTableVersion> {
                        key: SettingsKey::Version,
                        data: DB_VERSION_LATEST,
                    },
                ) {
                    Ok(_) => current_database_version = DB_VERSION_LATEST,
                    Err(e) => return Err(format!("Failed to set table version {}", e)),
                }
            } else {
                // TODO could use: PRAGMA user_version
                match TableSetting::<RecordMisc<SettingTableVersion>>::get(&conn) {
                    Ok(v) => current_database_version = v.data,
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

    type CreateTableFn = dyn Fn(&Connection) -> Result<bool, Error>;
    type UpdateTableFn = dyn Fn(&Connection, i64) -> Result<bool, Error>;
    let tables: Vec<(&str, &CreateTableFn, &UpdateTableFn)> = vec![
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
        (
            "TableFile",
            &TableFile::create_table,
            &TableFile::update_table,
        ),
        ("TableJob", &TableJob::create_table, &TableJob::update_table),
        (
            "TableJobMetadata",
            &TableJobMetadata::create_table,
            &TableJobMetadata::update_table,
        ),
        (
            "TableLabelPreset",
            &TableLabelPreset::create_table,
            &TableLabelPreset::update_table,
        ),
    ];

    // Create tables and update them if needed
    for (table_name, create_fn, update_fn) in tables.iter() {
        match create_fn(&conn) {
            Ok(created) => {
                if created {
                    info!("{} created", table_name);
                }
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
        match TableSetting::<RecordMisc<SettingTableVersion>>::update(
            &conn,
            &RecordMisc::<SettingTableVersion> {
                key: SettingsKey::Version,
                data: DB_VERSION_LATEST,
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
                ));
            }
        }
    }

    Ok(conn)
}

pub fn create_database(db_path: String) -> Result<rusqlite::Connection, String> {
    if let Err(e) = std::fs::create_dir_all(&db_path)
        && e.kind() != ErrorKind::AlreadyExists
    {
        return Err(format!("Failed to create database dir: {}", e));
    }
    let db_file = get_database_file(&db_path);
    static FIRST_RUN: Mutex<bool> = Mutex::new(true);

    match rusqlite::Connection::open(&db_file) {
        Ok(conn) => match FIRST_RUN.lock() {
            Ok(mut guard) => {
                info!(
                    "New database connection from thread: {:?}",
                    std::thread::current().id()
                );
                if enable_fk_constraints(&conn).expect("Foreign key constraints") != 0 {
                    unreachable!("Set foreign key constraints should return 0");
                }
                if *guard {
                    *guard = false;
                    info!("Open database at path: {}", db_file);
                    database_init(conn)
                } else {
                    Ok(conn)
                }
            }
            Err(e) => Err(format!("Failed to get database init lock: {}", e)),
        },
        Err(e) => Err(format!("Failed to open database: {}", e)),
    }
}

fn enable_fk_constraints(db: &Connection) -> Result<usize, rusqlite::Error> {
    db.execute("PRAGMA foreign_keys = ON", [])
}

pub fn backup_database(db: &Connection, path: String) -> Result<usize, rusqlite::Error> {
    db.execute("VACUUM main INTO ?1", params![path])
}

thread_local! {
    pub static DB: LazyLock<rusqlite::Connection> = LazyLock::new(|| {
        create_database(get_database_path()).expect("Attempt to open uninitialised database") // In separate function as rustfmt does not work inside this closure
    });
}

#[cfg(test)]
pub mod tests {
    use tempdir::TempDir;

    use crate::backend::database::db::{create_database, enable_fk_constraints};

    pub fn create_test_database() -> rusqlite::Connection {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        assert_eq!(enable_fk_constraints(&conn).unwrap(), 0);
        conn
    }

    fn check_foreign_keys_constraints(db: &rusqlite::Connection) {
        assert_eq!(
            db.pragma_query_value(None, "foreign_keys", |row| {
                let foreign_keys_enabled: isize = row.get(0).unwrap();
                Ok(foreign_keys_enabled)
            })
            .unwrap(),
            1, // Enabled
            "Expected foreign keys constraints to be enabled"
        );
    }

    #[test]
    fn create_database_temp_memory() {
        let db = create_test_database();
        check_foreign_keys_constraints(&db);
    }

    #[test]
    fn create_database_file() {
        let tmp_dir = TempDir::new("database_file").unwrap();
        let path = tmp_dir.path().display().to_string();
        let db = create_database(path).unwrap();
        check_foreign_keys_constraints(&db);
    }
}
