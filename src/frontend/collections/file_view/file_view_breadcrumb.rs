#[derive(Debug, PartialEq, Clone)]
pub struct Breadcrumb {
    pub current_index: isize,
    pub items_path: String,
    pub items: Vec<String>,
}

impl Breadcrumb {
    pub fn create_tail(mut current_path: String, separator: char) -> Vec<String> {
        if current_path.ends_with(separator) {
            current_path.pop(); // TODO avoid mut and use slice
        }
        let mut result = current_path
            .split(separator)
            .map(|p| p.to_string())
            .collect::<Vec<String>>();
        if let Some(first) = result.first_mut()
            && first.is_empty()
        {
            first.push(separator);
        }
        result
    }

    pub fn get_parent_dir(&self, separator: char) -> String {
        let new_index = self.current_index - 1;
        if new_index > 0 {
            return format!(
                "{}{}{}",
                separator,
                self.items[1..=new_index as usize].join(separator.to_string().as_str()),
                separator
            );
        }
        separator.to_string()
    }

    pub fn update(&mut self, new_path: String, separator: char) {
        if self.items_path.starts_with(&new_path) {
            self.current_index =
                (Self::create_tail(new_path.clone(), separator).len() - 1) as isize;
        } else {
            let tail = Self::create_tail(new_path.clone(), separator);
            self.current_index = (tail.len() - 1) as isize;
            self.items = tail;
            self.items_path = new_path;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path;

    use crate::frontend::collections::file_view::file_view_breadcrumb::Breadcrumb;

    #[test]
    fn test_breadcrumbs() {
        let linux_sep = path::MAIN_SEPARATOR;
        assert_eq!(
            Breadcrumb::create_tail("/".to_string(), linux_sep),
            vec!["/"]
        );
        assert_eq!(
            Breadcrumb::create_tail("/hello/123/abc/".to_string(), linux_sep),
            vec!["/", "hello", "123", "abc"]
        );
        assert_eq!(
            Breadcrumb::create_tail("/hello/123/abc".to_string(), linux_sep),
            vec!["/", "hello", "123", "abc"]
        );
        assert_eq!(
            Breadcrumb::create_tail("/hello/".to_string(), linux_sep),
            vec!["/", "hello"]
        );
    }

    #[test]
    fn test_parent_dir_breadcrumb() {
        const SEP: char = path::MAIN_SEPARATOR;
        let test_data = [
            (
                Breadcrumb {
                    items_path: "/test/abc/".to_string(),
                    current_index: 2,
                    items: Breadcrumb::create_tail("/test/abc/".to_string(), SEP),
                },
                "/test/".to_string(),
            ),
            (
                Breadcrumb {
                    items_path: "/test/abc/".to_string(),
                    current_index: 1,
                    items: Breadcrumb::create_tail("/test/abc/".to_string(), SEP),
                },
                "/".to_string(),
            ),
            (
                Breadcrumb {
                    items_path: "/test/".to_string(),
                    current_index: 1,
                    items: Breadcrumb::create_tail("/test/".to_string(), SEP),
                },
                "/".to_string(),
            ),
            (
                Breadcrumb {
                    items_path: "/first/second/third/forth/".to_string(),
                    current_index: 4,
                    items: Breadcrumb::create_tail("/first/second/third/forth/".to_string(), SEP),
                },
                "/first/second/third/".to_string(),
            ),
            (
                Breadcrumb {
                    items_path: "/first/second/third/forth/".to_string(),
                    current_index: 3,
                    items: Breadcrumb::create_tail("/first/second/third/forth/".to_string(), SEP),
                },
                "/first/second/".to_string(),
            ),
        ];
        for (bread, expected_path) in test_data {
            assert_eq!(bread.get_parent_dir(SEP), expected_path);
        }
    }

    #[test]
    fn test_update_breadcrumb() {
        const SEP: char = path::MAIN_SEPARATOR;
        let test_data = [
            (
                Breadcrumb {
                    items_path: "/test/abc/".to_string(),
                    current_index: 1,
                    items: vec!["/".to_string(), "test".to_string(), "abc".to_string()],
                },
                "/test/abc/xyz/".to_string(), // Move to new dir
                Breadcrumb {
                    items_path: "/test/abc/xyz/".to_string(),
                    current_index: 3,
                    items: vec![
                        "/".to_string(),
                        "test".to_string(),
                        "abc".to_string(),
                        "xyz".to_string(),
                    ],
                },
            ),
            (
                Breadcrumb {
                    items_path: "/test/abc/".to_string(),
                    current_index: 2,
                    items: vec!["/".to_string(), "test".to_string(), "abc".to_string()],
                },
                "/test/".to_string(), // Move to child in existing tree
                Breadcrumb {
                    items_path: "/test/abc/".to_string(),
                    current_index: 1,
                    items: vec!["/".to_string(), "test".to_string(), "abc".to_string()],
                },
            ),
            (
                Breadcrumb {
                    items_path: "/test/abc/".to_string(),
                    current_index: 2,
                    items: vec!["/".to_string(), "test".to_string(), "abc".to_string()],
                },
                "/new/branch/attempt/".to_string(), // Move to brand new tree
                Breadcrumb {
                    items_path: "/new/branch/attempt/".to_string(),
                    current_index: 3,
                    items: vec![
                        "/".to_string(),
                        "new".to_string(),
                        "branch".to_string(),
                        "attempt".to_string(),
                    ],
                },
            ),
            (
                Breadcrumb {
                    items_path: "/test/abc/".to_string(),
                    current_index: 0,
                    items: vec!["/".to_string(), "test".to_string(), "abc".to_string()],
                },
                "/new/branch/abc/".to_string(), // Move to brand new tree with same leaf name
                Breadcrumb {
                    items_path: "/new/branch/abc/".to_string(),
                    current_index: 3,
                    items: vec![
                        "/".to_string(),
                        "new".to_string(),
                        "branch".to_string(),
                        "abc".to_string(),
                    ],
                },
            ),
        ];
        for (mut initial, new_path, expected) in test_data {
            initial.update(new_path, SEP); // Apply directory change
            assert_eq!(initial, expected);
        }
    }
}
