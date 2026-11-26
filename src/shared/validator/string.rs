use std::rc::Rc;

use super::validator_base::Validator;

impl Validator<String> {
    pub fn with_not_empty(mut self) -> Self {
        self.check_fn
            .push(Rc::new(move |value: &String| -> Option<String> {
                if value.is_empty() {
                    return Some("Cannot be empty".to_string());
                }
                None
            }));
        self
    }

    pub fn no_duplicates(mut self, entries: Vec<String>) -> Self {
        self.check_fn
            .push(Rc::new(move |value: &String| -> Option<String> {
                if entries.iter().find(|e| *e == value).is_some() {
                    return Some(format!("'{}' already exists", value));
                }
                None
            }));
        self
    }

    pub fn with_expected_length(mut self, len: usize) -> Self {
        self.check_fn
            .push(Rc::new(move |value: &String| -> Option<String> {
                if value.len() != len {
                    return Some(format!("Expected length of: {}", len));
                }
                None
            }));
        self
    }

    pub fn with_max_length(mut self, len: usize) -> Self {
        self.check_fn
            .push(Rc::new(move |value: &String| -> Option<String> {
                if value.len() > len {
                    return Some(format!("Max length of: {}", len));
                }
                None
            }));
        self
    }

    pub fn with_only_allowed_chars(mut self, chars: &'static str) -> Self {
        self.check_fn
            .push(Rc::new(move |value: &String| -> Option<String> {
                for c in value.chars() {
                    if !chars.contains(c) {
                        return Some(format!("Found invalid character: {}", c));
                    }
                }
                None
            }));
        self
    }

    pub fn with_disallowed_chars(mut self, chars: &'static str) -> Self {
        self.check_fn
            .push(Rc::new(move |value: &String| -> Option<String> {
                for c in value.chars() {
                    if chars.contains(c) {
                        return Some(format!("Found invalid character: {}", c));
                    }
                }
                None
            }));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::validator::validator_base::{TValidator, Validator};

    #[test]
    fn expected_length() {
        assert!(
            Validator::<String>::create()
                .with_not_empty()
                .validate(&"test".to_string())
                .is_none(),
        );
        assert!(
            Validator::<String>::create()
                .with_not_empty()
                .validate(&"".to_string())
                .is_some(),
        );

        assert!(
            Validator::<String>::create()
                .no_duplicates(vec![])
                .validate(&"test".to_string())
                .is_none(),
        );
        assert!(
            Validator::<String>::create()
                .no_duplicates(vec!["a".to_string(), "b".to_string(), "TEST".to_string()])
                .validate(&"test".to_string())
                .is_none(),
        );
        assert!(
            Validator::<String>::create()
                .no_duplicates(vec![
                    "a".to_string(),
                    "test".to_string(),
                    "TEST".to_string()
                ])
                .validate(&"test".to_string())
                .is_some(),
        );

        assert!(
            Validator::<String>::create()
                .with_expected_length(5)
                .validate(&"test".to_string())
                .is_some(),
        );
        assert!(
            Validator::<String>::create()
                .with_expected_length(2)
                .validate(&"test".to_string())
                .is_some(),
        );
        assert!(
            Validator::<String>::create()
                .with_expected_length(1)
                .validate(&"test".to_string())
                .is_some(),
        );
        assert!(
            Validator::<String>::create()
                .with_expected_length(0)
                .validate(&"test".to_string())
                .is_some(),
        );
        assert!(
            Validator::<String>::create()
                .with_expected_length(4)
                .validate(&"test".to_string())
                .is_none(),
            "Falsely triggered failure"
        );
    }

    #[test]
    fn max_length() {
        assert!(
            Validator::<String>::create()
                .with_max_length(2)
                .validate(&"test".to_string())
                .is_some()
        );
        assert!(
            Validator::<String>::create()
                .with_max_length(10)
                .validate(&"test".to_string())
                .is_none(),
            "Falsely triggered failure"
        );
    }

    #[test]
    fn only_allowed_chars() {
        assert!(
            Validator::<String>::create()
                .with_only_allowed_chars("abc")
                .validate(&"test".to_string())
                .is_some()
        );
        assert!(
            Validator::<String>::create()
                .with_only_allowed_chars("abc")
                .validate(&"aa".to_string())
                .is_none(),
            "Falsely triggered failure"
        );
    }

    #[test]
    fn only_disallowed_chars() {
        assert!(
            Validator::<String>::create()
                .with_disallowed_chars("abc")
                .validate(&"test".to_string())
                .is_none()
        );
        assert!(
            Validator::<String>::create()
                .with_disallowed_chars("abc")
                .validate(&"aa".to_string())
                .is_some(),
            "Falsely triggered failure"
        );
    }
}
