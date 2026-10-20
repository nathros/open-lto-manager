use chrono::Local;

use crate::shared::models::database::{
    file::model_file::RecordFile,
    tape::model_tape::{
        EncryptionType, HardwareEncryptionType, RecordTape, SoftwareEncryptionType, TapeFormat,
    },
};

pub fn get_test_tape() -> RecordTape {
    RecordTape {
        id: 0,
        user_id: 1, // Admin
        manufacturer_id: 3,
        tape_type_id: 5,
        barcode: "123456".to_string(),
        serial: None,
        format: TapeFormat::LTFS,
        worm: false,
        encryption_type: EncryptionType::None,
        encryption_sw: SoftwareEncryptionType::None,
        encryption_hw: HardwareEncryptionType::None,
        compressed: false,
        used_space: 0,
        created: Local::now(),
        last_used: Local::now(),
    }
}

pub fn get_file_test_data_1(tape_id: i64) -> Vec<RecordFile> {
    let file = RecordFile {
        id: 0,
        parent_id: 0,
        user_id: 1,
        tape_id: Some(tape_id),
        file_name_virt: "".to_string(),
        file_path_virt: None,
        file_name_phy: "".to_string(),
        file_path_phy: "".to_string(),
        file_size: 0,
        created: Local::now(),
        modified: Local::now(),
        hash: "".to_string(),
        icon: "".to_string(),
    };

    let dir: RecordFile = RecordFile {
        id: 0,
        parent_id: 0,
        user_id: 1,
        tape_id: None,
        file_name_virt: "".to_string(),
        file_path_virt: Some("".to_string()),
        file_name_phy: "".to_string(),
        file_path_phy: "".to_string(),
        file_size: 0,
        created: Local::now(),
        modified: Local::now(),
        hash: "".to_string(),
        icon: "".to_string(),
    };

    vec![
        RecordFile {
            // id: 2
            parent_id: 1,
            file_name_virt: "Test dir".to_string(),
            file_path_virt: Some("/Test dir/".to_string()),
            ..dir.clone()
        },
        RecordFile {
            // id: 3
            parent_id: 1,
            file_name_virt: "Empty dir".to_string(),
            file_path_virt: Some("/Empty dir/".to_string()),
            ..dir.clone()
        },
        RecordFile {
            // id: 4
            parent_id: 1,
            file_name_virt: "Documents".to_string(),
            file_path_virt: Some("/Documents/".to_string()),
            ..dir.clone()
        },
        RecordFile {
            // id: 5
            parent_id: 4,
            file_name_virt: "txt".to_string(),
            file_path_virt: Some("/Documents/txt/".to_string()),
            ..dir.clone()
        },
        RecordFile {
            // id: 6
            parent_id: 4,
            file_name_virt: "json".to_string(),
            file_path_virt: Some("/Documents/json/".to_string()),
            ..dir.clone()
        },
        RecordFile {
            // id: 7
            parent_id: 6,
            file_name_virt: "Batch1".to_string(),
            file_path_virt: Some("/Documents/json/Batch1/".to_string()),
            ..dir.clone()
        },
        RecordFile {
            // id: 8
            parent_id: 6,
            file_name_virt: "Batch2".to_string(),
            file_path_virt: Some("/Documents/json/Batch2/".to_string()),
            ..dir.clone()
        },
        // Files
        RecordFile {
            parent_id: 2,
            file_name_virt: "first_file.zip".to_string(),
            file_size: 100,
            ..file.clone()
        },
        RecordFile {
            parent_id: 5, // /Documents/txt/
            file_name_virt: "first.txt".to_string(),
            file_size: 100,
            ..file.clone()
        },
        RecordFile {
            parent_id: 5, // /Documents/txt/
            file_name_virt: "second.txt".to_string(),
            file_size: 300,
            ..file.clone()
        },
        RecordFile {
            parent_id: 5, // /Documents/txt/
            file_name_virt: "third.txt".to_string(),
            file_size: 400,
            ..file.clone()
        },
        RecordFile {
            parent_id: 5, // /Documents/txt/
            file_name_virt: "4th.txt".to_string(),
            file_size: 50,
            ..file.clone()
        },
        RecordFile {
            parent_id: 6, // /Documents/json/
            file_name_virt: "1.json".to_string(),
            file_size: 333,
            ..file.clone()
        },
        RecordFile {
            parent_id: 6, // /Documents/json/
            file_name_virt: "2.json".to_string(),
            file_size: 333,
            ..file.clone()
        },
        RecordFile {
            parent_id: 6, // /Documents/json/
            file_name_virt: "3.json".to_string(),
            file_size: 333,
            ..file.clone()
        },
        RecordFile {
            parent_id: 7, // /Documents/json/Batch1/
            file_name_virt: "a.json".to_string(),
            file_size: 333,
            ..file.clone()
        },
        RecordFile {
            parent_id: 8, // /Documents/json/Batch2/
            file_name_virt: "b.json".to_string(),
            file_size: 333,
            ..file.clone()
        },
        RecordFile {
            parent_id: 1, // /
            file_name_virt: "base.png".to_string(),
            file_size: 666,
            ..file.clone()
        },
        RecordFile {
            parent_id: 1, // /
            file_name_virt: "another.jpg".to_string(),
            file_size: 666,
            ..file.clone()
        },
    ]
}
