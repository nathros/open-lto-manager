use super::icons::Icons;

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
            Level::Error => Icons::ERROR,
            Level::Warning => Icons::WARNING,
            Level::Info => Icons::INFO,
            Level::Success => Icons::SUCCESS,
        }
    }
}
