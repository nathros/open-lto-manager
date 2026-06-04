#[derive(PartialEq, Clone)]
pub enum Level {
    Error,
    Warning,
    Info,
    Success,
}

impl Level {
    pub fn to_class(&self) -> &str {
        match self {
            Level::Error => "error",
            Level::Warning => "warning",
            Level::Info => "info",
            Level::Success => "success",
        }
    }
}
