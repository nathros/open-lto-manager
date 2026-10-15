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
    pub is_dir: bool,
    pub is_selected: bool,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub created: DateTime<Local>,
    pub modified: DateTime<Local>,
    pub icon: String,
}

impl FileViewInfo {
    pub fn is_dir(file_name: &str) -> bool {
        file_name.chars().last().is_some_and(|c| c == '/')
    }

    pub fn trim_name(file_name: String) -> String {
        if file_name.chars().next().is_some_and(|c| c == '/')
            && file_name.chars().last().is_some_and(|c| c == '/')
            && file_name.len() > 1
        {
            file_name.as_str()[1..file_name.len() - 1].to_string()
        } else {
            file_name
        }
    }

    pub fn pretty_size(&self) -> String {
        let mut size = self.file_size as f32;
        if size < 1024.0 {
            if size == 0.0 {
                return "empty".to_string();
            }
            return format!("{} bytes", self.file_size);
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
    fn check_is_dir() {
        assert!(FileViewInfo::is_dir("/"));
        assert!(FileViewInfo::is_dir("/test/"));
        assert!(!FileViewInfo::is_dir("test"));
        assert!(FileViewInfo::is_dir("test/"));
        assert!(FileViewInfo::is_dir("/test/"));
    }

    #[test]
    fn check_trim_name() {
        assert_eq!(FileViewInfo::trim_name("/dir/".to_string()), "dir");
        assert_eq!(FileViewInfo::trim_name("file".to_string()), "file");
        assert_eq!(FileViewInfo::trim_name("/".to_string()), "/");
    }

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
            assert_eq!(
                FileViewInfo {
                    file_size: *size,
                    ..Default::default()
                }
                .pretty_size(),
                str.to_string()
            );
        });
    }
}
