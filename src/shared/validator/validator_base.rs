use std::rc::Rc;

pub trait TValidator<T, R> {
    fn create() -> R;
    fn validate(&self, value: &T) -> Option<String>;
}

pub type CheckFnType<T> = Rc<dyn Fn(&T) -> Option<String>>;

#[derive(Clone)]
pub struct Validator<T> {
    pub check_fn: Vec<CheckFnType<T>>,
}

impl<T> PartialEq for Validator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.check_fn
            .iter()
            .zip(other.check_fn.iter())
            .filter(|&(a, b)| !Rc::ptr_eq(a, b))
            .count()
            == 0
    }
}

impl<T> TValidator<T, Validator<T>> for Validator<T> {
    fn validate(&self, value: &T) -> Option<String> {
        for function in self.check_fn.iter().as_ref() {
            let result = function(value);
            if result.is_some() {
                return result;
            }
        }
        None
    }

    fn create() -> Validator<T> {
        Validator::<T> { check_fn: vec![] }
    }
}
