use num_traits::int::PrimInt;
use std::fmt::Debug;
use tracing::error;

use super::validator::Validator;

impl<D: PrimInt + Debug> Validator<D> {
    /*pub fn with_min_number(mut self, num: usize) -> Self {
        self.check_fn
            .push(Box::new(move |value: &D| -> Option<String> {
                if let Some(number) = D::from(num)
                    && number > *value
                {
                    return Some(format!("Minimum of: {}", num));
                }
                None
            }));
        self
    }*/

    pub fn with_min_number(mut self, num: D) -> Self {
        let number = num.to_i64().unwrap_or_else(|| {
            error!("Failed with_min_number: {:#?}", num);
            i64::MIN
        });
        self.check_fn
            .push(Box::new(move |value: &D| -> Option<String> {
                if let Some(check) = value.to_i64()
                    && check < number
                {
                    return Some(format!("Minimum of: {}", number));
                }
                None
            }));
        self
    }

    pub fn with_max_number(mut self, num: D) -> Self {
        let number = num.to_i64().unwrap_or_else(|| {
            error!("Failed with_max_number: {:#?}", num);
            i64::MIN
        });
        self.check_fn
            .push(Box::new(move |value: &D| -> Option<String> {
                if let Some(check) = value.to_i64()
                    && check > number
                {
                    return Some(format!("Maximim of: {}", number));
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
    fn min_number() {
        let validate = Validator::<i32>::create().with_min_number(50);
        assert!(validate.validate(&22_i32).is_some());
        assert!(validate.validate(&50_i32).is_none());
        assert!(validate.validate(&66_i32).is_none());

        let validate = Validator::<u8>::create().with_min_number(50);
        assert!(validate.validate(&22_u8).is_some());
        assert!(validate.validate(&50_u8).is_none());
        assert!(validate.validate(&66_u8).is_none());
    }

    #[test]
    fn max_number() {
        let validate = Validator::<i32>::create().with_max_number(50);
        assert!(validate.validate(&22_i32).is_none());
        assert!(validate.validate(&50_i32).is_none());
        assert!(validate.validate(&66_i32).is_some());

        let validate = Validator::<u8>::create().with_max_number(50);
        assert!(validate.validate(&22_u8).is_none());
        assert!(validate.validate(&50_u8).is_none());
        assert!(validate.validate(&66_u8).is_some());
    }
}
