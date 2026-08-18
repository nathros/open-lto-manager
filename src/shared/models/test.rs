#[cfg(test)]
pub mod tests {
    use enum_iterator::{Sequence, all};
    use std::fmt::Debug;

    pub fn from_generic_keys_test<T>(convert: &dyn Fn(&T) -> i64)
    where
        T: Sequence + Eq + Debug + From<i64>,
    {
        let all_settings: Vec<T> = all::<T>().collect::<Vec<_>>();
        all_settings.iter().for_each(|setting| {
            assert_eq!(
                *setting,
                T::from(convert(setting)), // Unable to find #[repr(i64)] trait, so cast is done outside
                "impl From<i64> for Enum {{}} is missing an entry"
            );
        });
    }
}
