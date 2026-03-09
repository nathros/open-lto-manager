#[derive(PartialEq, Clone)]
pub enum Level {
    Error,
    Warning,
    Info,
    Success,
}

impl Level {
    pub fn to_style(&self) -> &str {
        match self {
            Level::Error => "background-color:red",
            Level::Warning => "background-color:orange",
            Level::Info => "background-color:blue",
            Level::Success => "background-color:green",
        }
    }
}
