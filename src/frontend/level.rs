use super::icons::Icons;

#[derive(PartialEq, Eq, Clone)]
pub enum Level {
    Error,
    Warning,
    Info,
    Success,
}

impl Level {
    pub const fn to_class(&self) -> &str {
        match self {
            Level::Error => Icons::ERROR,
            Level::Warning => Icons::WARNING,
            Level::Info => Icons::INFO,
            Level::Success => Icons::SUCCESS,
        }
    }
}
