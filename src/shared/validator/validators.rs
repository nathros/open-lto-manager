use crate::shared::{
    r#const::Const,
    validator::validator_base::{TValidator, Validator},
};

pub struct Validators;

impl Validators {
    pub fn virtual_dir_name(existing: Vec<String>) -> Validator<String> {
        Validator::<String>::create()
            .with_not_empty()
            .with_max_length(Const::FILENAME_MAX_LEN)
            .with_disallowed_chars(Const::FILENAME_DISALLOW)
            .no_duplicates(existing)
    }
}
