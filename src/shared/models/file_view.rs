use dioxus::fullstack::serde::{Deserialize, Serialize};

use crate::backend::api::api_file_view::fv_files_in_dir;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FileView {
    pub is_dir: bool,
    pub nest: usize,
    pub path: String,
    pub name: String,
    pub size: u64,
    pub expanded: bool,
    pub hidden: bool,
    pub selected: bool,
}

impl FileView {
    pub async fn toggle_dir(dir: &mut Vec<FileView>, index: usize) {
        if let Some(file) = dir.get_mut(index) {
            file.expanded = !file.expanded;
            let expanded = file.expanded;

            if file.expanded && file.size == 0 {
                let fetch = fv_files_in_dir(file.path.clone(), false, file.nest + 1).await;
                if let Ok(Ok(new_dir)) = fetch {
                    FileView::insert_dir(dir, new_dir, index);
                }
            } else {
                let nest = file.nest;
                for element in dir.iter_mut().skip(index + 1) {
                    if element.nest > nest {
                        element.hidden = !expanded;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    pub fn insert_dir(
        main: &mut Vec<FileView>,
        new_dir: Vec<FileView>,
        index: usize,
    ) -> Option<&str> {
        #[cfg(debug_assertions)]
        FileView::println(main);

        let current_dir = main.get(index);
        if current_dir.is_none() {
            return Some("Index does not exist");
        }
        if let Some(element) = main.get(index) {
            if !element.is_dir {
                return Some("Index is not dir");
            }
            let new_dir_size = new_dir.len();
            let mut tmp = main.split_off(index + 1); // Split elements after dir into separate vector
            if let Some(last) = main.last_mut() {
                last.size = new_dir_size as u64; // Set dir size as number of elements
            }
            main.extend_from_slice(&new_dir); // Append new items
            main.append(&mut tmp); // Append split
        }

        None
    }

    pub fn println(list: &[FileView]) {
        for (index, file) in list.iter().enumerate() {
            println!(
                "{}",
                format_args!(
                    "[{:>2}]:{:>2} {:>5} {:>5}  {}",
                    index, file.nest, file.is_dir, file.size, file.name
                )
            );
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use crate::{
        backend::api::api_file_view::{fv_files_in_dir, fv_working_dir},
        shared::models::file_view::FileView,
    };

    #[tokio::test]
    async fn invalid_ranges_insert() {
        let test_dir_str = fv_working_dir().await.unwrap(); // Will be code dir
        let mut root_dir = fv_files_in_dir(test_dir_str.clone(), false, 0)
            .await
            .unwrap()
            .unwrap();
        let root_dir_len = root_dir.len();

        let result = FileView::insert_dir(&mut root_dir, vec![], root_dir_len + 1);
        assert!(result.is_some(), "Out of range not triggered");

        let (dir_index, _) = root_dir
            .iter()
            .enumerate()
            .find(|(_, f)| f.name == "src")
            .unwrap();
        let result = FileView::insert_dir(&mut root_dir, vec![], dir_index);
        assert!(result.is_none(), "Should be valid dir");

        let (file_index, _) = root_dir
            .iter()
            .enumerate()
            .find(|(_, f)| f.name == "Cargo.toml")
            .unwrap();
        let result = FileView::insert_dir(&mut root_dir, vec![], file_index);
        assert!(result.is_some(), "Should be invalid file");
    }

    #[tokio::test]
    async fn insert_dir() {
        let test_dir_str = fv_working_dir().await.unwrap(); // Will be code dir
        let mut root_dir = fv_files_in_dir(test_dir_str.clone(), false, 0)
            .await
            .unwrap()
            .unwrap();

        // Search for src dir which will be used for test
        let (test_index_dir, src_dir) = root_dir
            .iter()
            .enumerate()
            .find(|(_, f)| f.name == "src")
            .unwrap();

        assert!(
            test_index_dir > 0,
            "Expected to find a dir in the second half"
        );

        let test_inner_dir_str = format!(
            "{}{}{}",
            test_dir_str,
            std::path::MAIN_SEPARATOR,
            src_dir.name
        );

        let src_dir = fv_files_in_dir(test_inner_dir_str, false, 1)
            .await
            .unwrap()
            .unwrap();

        let original_len = root_dir.len();
        let additional_len = src_dir.len();

        // println!("Before");
        // println!("root_dir [{}]:", test_index_dir);
        // FileView::println(&root_dir);
        // println!("------------------");
        // println!("src_dir:");
        // FileView::println(&src_dir);
        // println!("==================");

        assert_eq!(
            FileView::insert_dir(&mut root_dir, src_dir, test_index_dir),
            None // No error
        );
        assert_eq!(
            root_dir.len(),
            (original_len + additional_len),
            "Expected length mismatch"
        );

        // println!("After");
        // println!("root_dir [{}]:", test_index_dir);
        // FileView::println(&root_dir);
        // println!("==================");
    }
}
