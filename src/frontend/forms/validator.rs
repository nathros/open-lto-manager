pub trait TValidator<T, R> {
    fn create() -> R;
    fn validate(&self, value: &T) -> Option<String>;
}

pub type CheckFnType<T> = Box<dyn Fn(&T) -> Option<String>>;

pub struct Validator<T> {
    pub check_fn: Vec<CheckFnType<T>>,
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
