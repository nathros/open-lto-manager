use super::validator::Validator;

impl Validator<String> {
    pub fn with_expected_length(mut self, len: usize) -> Self {
        self.check_fn
            .push(Box::new(move |value: &String| -> Option<String> {
                if value.len() != len {
                    return Some(format!("Expected length of: {}", len));
                }
                None
            }));
        self
    }

    pub fn with_max_length(mut self, len: usize) -> Self {
        self.check_fn
            .push(Box::new(move |value: &String| -> Option<String> {
                if value.len() > len {
                    return Some(format!("Max length of: {}", len));
                }
                None
            }));
        self
    }

    pub fn with_only_allowed_chars(mut self, chars: &'static str) -> Self {
        self.check_fn
            .push(Box::new(move |value: &String| -> Option<String> {
                for c in value.chars() {
                    if !chars.contains(c) {
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
    use crate::frontend::forms::validator::{TValidator, Validator};

    #[test]
    fn expected_length() {
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
}
