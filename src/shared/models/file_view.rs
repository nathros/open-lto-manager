use std::cmp::Ordering;

use chrono::{DateTime, Local};
use dioxus::fullstack::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct FileView {
    pub current_dir: Option<FileViewInfo>,
    pub children: Vec<FileViewInfo>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct FileViewInfo {
    pub id: i64,
    pub is_selected: bool,
    pub is_dir: bool,
    pub file_name: String,
    pub file_size: i64,
    pub file_size_pretty: String,
    pub created: DateTime<Local>,
    pub modified: DateTime<Local>,
    pub icon: String,
}

impl FileViewInfo {
    pub fn sort(children: &mut [FileViewInfo]) {
        children.sort_by(|a, b| {
            if a.is_dir == b.is_dir {
                a.file_name.cmp(&b.file_name)
            } else if a.is_dir {
                Ordering::Less // Directories at top
            } else {
                Ordering::Greater
            }
        });
    }

    #[cfg(feature = "server")]
    pub fn pretty_size(file_size: i64) -> String {
        let mut size = file_size as f32;
        if size < 1024.0 {
            if size == 0.0 {
                return "empty".to_string();
            }
            return format!("{} bytes", file_size);
        }
        for unit in ["KB", "MB", "GB", "TB"] {
            size /= 1024.0;
            if size < 1024.0 {
                return format!("{:.2} {}", size, unit);
            }
        }
        format!("{:.2} TB", size)
    }

    #[cfg(feature = "server")]
    pub fn default_dir_icon() -> String {
        "#folder".to_string()
    }

    #[cfg(feature = "server")]
    fn dir_icon(icon: String) -> String {
        if icon.is_empty() {
            Self::default_dir_icon() // Default for dir
        } else {
            icon
        }
    }

    #[cfg(feature = "server")]
    fn file_icon(file_name: &String) -> String {
        use std::path::Path;
        [
            "#", // Anchor for sprite
            &Path::new(&file_name)
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase(),
        ]
        .concat()
    }

    #[cfg(feature = "server")]
    pub fn get_icon(is_dir: bool, file_name: &String, icon: String) -> String {
        if is_dir {
            Self::dir_icon(icon)
        } else {
            Self::file_icon(file_name)
        }
    }

    #[cfg(feature = "server")]
    pub fn get_icon_default(is_dir: bool, file_name: &String) -> String {
        if is_dir {
            Self::default_dir_icon()
        } else {
            Self::file_icon(file_name)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::models::file_view::FileViewInfo;

    #[test]
    fn check_pretty_size() {
        let test_data = [
            (0, "empty"),
            (888, "888 bytes"),
            (1024, "1.00 KB"),
            (1025, "1.00 KB"),
            (1099, "1.07 KB"),
            (1048576, "1.00 MB"),
            (345742486, "329.73 MB"),
            (1073741824, "1.00 GB"),
            (406643380416, "378.72 GB"),
            (1099511627776, "1.00 TB"),
            (1125899906842624, "1024.00 TB"), // No PB tape exists
            (9223372036854775807, "8388608.00 TB"),
        ];

        test_data.iter().for_each(|(size, str)| {
            assert_eq!(FileViewInfo::pretty_size(*size), str.to_string());
        });
    }
}
