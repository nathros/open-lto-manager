#[cfg(feature = "server")]
pub fn setup_logging() -> Result<(), String> {
    use crate::backend::env::{get_logging_file, get_logging_path};
    use std::fs::OpenOptions;
    use std::io::ErrorKind;
    use tracing::Level;
    use tracing_subscriber::{filter, fmt, layer::Layer, prelude::*, Registry};

    let log_file_path = get_logging_path();
    let log_file = get_logging_file();

    match std::fs::remove_file(&log_file) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => {
            return Err(format!(
                "Unexpected error with log file: {}, with error: {}",
                log_file, e
            ))
        }
    }
    match std::fs::create_dir_all(&log_file_path) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() != ErrorKind::AlreadyExists {
                return Err(format!(
                    "Failed to create logging dir: {} with error: {}",
                    log_file_path, e
                ));
            }
        }
    }
    let file_result = OpenOptions::new().create(true).append(true).open(&log_file);
    if let Some(e) = file_result.as_ref().err() {
        return Err(format!(
            "Failed to create log file: {} with error: {}",
            log_file, e
        ));
    }

    let subscriber = Registry::default()
        .with(
            // stdout layer
            fmt::layer()
                .compact()
                .with_ansi(true)
                .with_filter(filter::LevelFilter::from_level(Level::INFO)),
        )
        .with(
            // File layer, TODO log rotation
            fmt::layer()
                .with_ansi(false)
                .with_writer(file_result.unwrap())
                .with_filter(filter::LevelFilter::from_level(Level::INFO)),
        );

    if let Some(e) = tracing::subscriber::set_global_default(subscriber).err() {
        return Err(format!("Failed to set global logger: {}", e));
    }
    Ok(())
}
