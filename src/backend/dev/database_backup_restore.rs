use chrono::{DateTime, Local};
use serde_json::Value;
use std::{
    fs::{self, read_dir},
    path::Path,
};
use tracing::{error, info};

use crate::{
    backend::{
        database::{
            db::{DB, backup_database},
            tables::{
                file::table_file::TableFile,
                table::{RecordInsert, TableClear, TableCreate},
                tape::table_tape::TableTape,
            },
        },
        env::get_database_path,
    },
    shared::models::database::{
        file::model_file::RecordFile,
        tape::model_tape::{
            EncryptionType, HardwareEncryptionType, RecordTape, SoftwareEncryptionType, TapeFormat,
        },
    },
};

pub fn dev_database_list() -> Vec<String> {
    let path = get_database_path();
    let mut results = vec![];

    let read_dir = match read_dir(&path) {
        Ok(dir) => dir,
        Err(_e) => {
            return results;
        }
    };

    for current_path in read_dir.flatten() {
        if let Ok(metadata) = current_path.metadata()
            && metadata.is_dir()
        {
            results.push(current_path.file_name().into_string().unwrap_or_default());
        }
    }

    results
}

pub fn dev_database_backup(dir: String) -> bool {
    DB.with(|db| {
        let path = format!("{}/{}", get_database_path(), dir);
        let path_file = format!("{}/{}/result.json", get_database_path(), dir);
        let path_file_backup = format!("{}/{}/database.db", get_database_path(), dir);

        let base_data_str = r#"{
            "tapes": [],
            "files": []
        }"#;

        let mut json: Value = serde_json::from_str(base_data_str).unwrap_or(Value::Null);

        let tapes = match TableTape::get_all(db) {
            Ok(records) => records,
            Err(e) => {
                error!("{}", e);
                return false;
            }
        };
        if let Some(array) = json["tapes"].as_array_mut() {
            for t in tapes {
                let mut tmp = Value::Object(serde_json::Map::new());
                tmp["id"] = Value::Number(t.id.into());
                tmp["manufacturer_id"] = Value::Number(t.manufacturer_id.into());
                tmp["tape_type_id"] = Value::Number(t.tape_type_id.into());
                tmp["barcode"] = Value::String(t.barcode);
                tmp["serial"] = Value::String(t.serial);
                tmp["format"] = Value::Number((t.format as i64).into());
                tmp["worm"] = Value::Bool(t.worm);
                tmp["encryption_type"] = Value::Number((t.encryption_type as i64).into());
                tmp["encryption_sw"] = Value::Number((t.encryption_sw as i64).into());
                tmp["encryption_hw"] = Value::Number((t.encryption_hw as i64).into());
                tmp["compressed"] = Value::Bool(t.compressed);
                tmp["used_space"] = Value::Number(t.used_space.into());
                tmp["created"] = Value::String(t.created.to_rfc3339());
                tmp["last_used"] = Value::String(t.created.to_rfc3339());
                array.push(tmp);
            }
        } else {
            return false;
        }

        let files = match TableFile::get_all(db) {
            Ok(records) => records,
            Err(e) => {
                error!("{}", e);
                return false;
            }
        };
        if let Some(array) = json["files"].as_array_mut() {
            for t in files {
                let mut tmp = Value::Object(serde_json::Map::new());
                tmp["id"] = Value::Number(t.id.into());
                tmp["tape_id"] = Value::Number(t.tape_id.unwrap_or(0).into());
                tmp["file_name_virt"] = Value::String(t.file_name_virt);
                tmp["file_path_virt"] = Value::String(t.file_path_virt);
                tmp["file_name_phy"] = Value::String(t.file_name_phy);
                tmp["file_path_phy"] = Value::String(t.file_path_phy);
                tmp["file_size"] = Value::Number(t.file_size.into());
                tmp["created"] = Value::String(t.created.to_rfc3339());
                tmp["modified"] = Value::String(t.modified.to_rfc3339());
                tmp["hash"] = Value::String(t.hash);
                tmp["icon"] = Value::String(t.icon);
                array.push(tmp);
            }
        } else {
            return false;
        }

        if Path::new(&path).exists() {
            match std::fs::remove_dir_all(&path) {
                Ok(_) => info!("Index {} already existed, now removed", dir),
                Err(e) => {
                    error!("1 {}", e);
                    return false;
                }
            }
        }

        match std::fs::create_dir_all(&path) {
            Ok(_) => match fs::write(path_file, json.to_string()) {
                Ok(_f) => {}
                Err(e) => {
                    error!("2 {}", e);
                    return false;
                }
            },
            Err(e) => {
                error!("3 {}", e);
                return false;
            }
        }

        if let Err(e) = backup_database(db, path_file_backup) {
            error!("Backup error {}", e);
            return false;
        }

        true
    })
}

pub fn dev_database_restore(dir: String) -> Option<bool> {
    DB.with(|db| {
        let path_file = format!("{}/{}/result.json", get_database_path(), dir);

        let mut tapes = vec![];
        let mut files = vec![];
        match std::fs::read_to_string(path_file) {
            Ok(json_str) => {
                let json: Value = serde_json::from_str(json_str.as_str()).unwrap_or(Value::Null);
                if let Some(array) = json["tapes"].as_array() {
                    for value in array {
                        tapes.push(RecordTape {
                            id: match value["id"].as_number() {
                                Some(i) => i.as_i64()?,
                                None => {
                                    error!("Tape failure id");
                                    return None;
                                }
                            },

                            manufacturer_id: match value["manufacturer_id"].as_number() {
                                Some(i) => i.as_i64()?,
                                None => {
                                    error!("Tape failure manufacturer_id");
                                    return None;
                                }
                            },

                            tape_type_id: match value["tape_type_id"].as_number() {
                                Some(i) => i.as_i64()?,
                                None => {
                                    error!("Tape failure tape_type_id");
                                    return None;
                                }
                            },

                            barcode: match value["barcode"].as_str() {
                                Some(i) => i.to_string(),
                                None => {
                                    error!("Tape failure barcode");
                                    return None;
                                }
                            },

                            serial: match value["serial"].as_str() {
                                Some(i) => i.to_string(),
                                None => {
                                    error!("Tape failure serial");
                                    return None;
                                }
                            },

                            format: match value["format"].as_number() {
                                Some(i) => TapeFormat::from(i.as_i64()?),
                                None => {
                                    error!("Tape failure format");
                                    return None;
                                }
                            },

                            worm: match value["worm"].as_bool() {
                                Some(i) => i,
                                None => {
                                    error!("Tape failure worm");
                                    return None;
                                }
                            },

                            encryption_type: match value["encryption_type"].as_number() {
                                Some(i) => EncryptionType::from(i.as_i64()?),
                                None => {
                                    error!("Tape failure encryption_type");
                                    return None;
                                }
                            },

                            encryption_sw: match value["encryption_sw"].as_number() {
                                Some(i) => SoftwareEncryptionType::from(i.as_i64()?),
                                None => {
                                    error!("Tape failure encryption_sw");
                                    return None;
                                }
                            },

                            encryption_hw: match value["encryption_hw"].as_number() {
                                Some(i) => HardwareEncryptionType::from(i.as_i64()?),
                                None => {
                                    error!("Tape failure encryption_hw");
                                    return None;
                                }
                            },

                            compressed: match value["compressed"].as_bool() {
                                Some(i) => i,
                                None => {
                                    error!("Tape failure compressed");
                                    return None;
                                }
                            },

                            used_space: match value["used_space"].as_number() {
                                Some(i) => i.as_i64()?,
                                None => {
                                    error!("Tape failure used_space");
                                    return None;
                                }
                            },

                            created: match value["created"].as_str() {
                                Some(i) => DateTime::parse_from_rfc3339(i)
                                    .unwrap_or(Local::now().into())
                                    .into(),
                                None => {
                                    error!("Tape failure created");
                                    return None;
                                }
                            },

                            last_used: match value["last_used"].as_str() {
                                Some(i) => DateTime::parse_from_rfc3339(i)
                                    .unwrap_or(Local::now().into())
                                    .into(),
                                None => {
                                    error!("Tape failure last_used");
                                    return None;
                                }
                            },
                        });
                    }
                }

                if let Some(array) = json["files"].as_array() {
                    for value in array {
                        files.push(RecordFile {
                            id: match value["id"].as_number() {
                                Some(i) => i.as_i64()?,
                                None => {
                                    error!("File failure id");
                                    return None;
                                }
                            },
                            tape_id: match value["tape_id"].as_number() {
                                Some(i) => {
                                    let result = i.as_i64()?;
                                    if result == 0 { None } else { Some(result) }
                                }
                                None => {
                                    error!("File failure tape_id");
                                    return None;
                                }
                            },
                            file_name_virt: match value["file_name_virt"].as_str() {
                                Some(i) => i.to_string(),
                                None => {
                                    error!("File failure file_name_virt");
                                    return None;
                                }
                            },
                            file_path_virt: match value["file_path_virt"].as_str() {
                                Some(i) => i.to_string(),
                                None => {
                                    error!("File failure file_path_virt");
                                    return None;
                                }
                            },
                            file_name_phy: match value["file_name_phy"].as_str() {
                                Some(i) => i.to_string(),
                                None => {
                                    error!("File failure file_name_phy");
                                    return None;
                                }
                            },
                            file_path_phy: match value["file_path_phy"].as_str() {
                                Some(i) => i.to_string(),
                                None => {
                                    error!("File failure file_path_phy");
                                    return None;
                                }
                            },
                            file_size: match value["file_size"].as_number() {
                                Some(i) => i.as_i64()?,
                                None => {
                                    error!("File failure file_size");
                                    return None;
                                }
                            },
                            created: match value["created"].as_str() {
                                Some(i) => DateTime::parse_from_rfc3339(i)
                                    .unwrap_or(Local::now().into())
                                    .into(),
                                None => {
                                    error!("File failure created");
                                    return None;
                                }
                            },
                            modified: match value["modified"].as_str() {
                                Some(i) => DateTime::parse_from_rfc3339(i)
                                    .unwrap_or(Local::now().into())
                                    .into(),
                                None => {
                                    error!("File failure modified");
                                    return None;
                                }
                            },
                            hash: match value["hash"].as_str() {
                                Some(i) => i.to_string(),
                                None => {
                                    error!("File failure hash");
                                    return None;
                                }
                            },
                            icon: match value["icon"].as_str() {
                                Some(i) => i.to_string(),
                                None => {
                                    error!("File failure icon");
                                    return None;
                                }
                            },
                        });
                    }
                }

                let clear_files = TableFile::clear_table(db);
                if let Err(e) = clear_files {
                    error!("File clear error {}", e);
                    return None;
                }
                // Got all records, now insert. Order matters
                match TableTape::create_table(db) {
                    Ok(_) => {
                        for t in tapes {
                            if let Err(e) = TableTape::insert(db, &t) {
                                error!("Tape insert error {}", e);
                                return None;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Tape clear error {}", e);
                        return None;
                    }
                }
                for t in files {
                    if let Err(e) = TableFile::insert(db, &t) {
                        error!("File insert error {}", e);
                        return None;
                    }
                }
            }
            Err(e) => {
                error!("1 {}", e);
                return None;
            }
        }

        Some(true)
    })
}
