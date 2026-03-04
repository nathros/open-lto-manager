#[derive(PartialEq, Clone)]
pub enum Level {
    Error,
    Warning,
    Info,
    Success,
}

pub fn level_style(level: Level) -> String {
    match level {
        Level::Error => "background-color:red".to_string(),
        Level::Warning => "background-color:orange".to_string(),
        Level::Info => "background-color:blue".to_string(),
        Level::Success => "background-color:green".to_string(),
    }
}
