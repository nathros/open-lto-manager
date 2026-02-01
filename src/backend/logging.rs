use crate::backend::env::{get_console_log_enabled, get_logging_file, get_logging_path};
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::sync::LazyLock;
use tracing::{error, level_filters::LevelFilter, Level};
use tracing_subscriber::{
    filter::{self, Filtered},
    fmt,
    layer::Layer,
    prelude::*,
    reload::{self, Handle},
    Registry,
};

pub type ReloadableLayer = Filtered<Box<dyn Layer<Registry> + Send + Sync>, LevelFilter, Registry>;

pub static FILE_LOG: LazyLock<Result<Handle<ReloadableLayer, Registry>, String>> =
    LazyLock::new(|| setup_logging());

pub fn change_file_logger_level(level: LevelFilter) -> bool {
    match FILE_LOG.as_ref() {
        Ok(log_file_layer) => match log_file_layer.modify(|layer| *layer.filter_mut() = level) {
            Ok(_) => true,
            Err(e) => {
                error!("Failed to update logger level: {}", e);
                false
            }
        },
        Err(e) => {
            error!("Cannot update level of uninitialised logger: {}", e);
            false
        }
    }
}

fn setup_logging() -> Result<Handle<ReloadableLayer, Registry>, String> {
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

    let file = file_result.unwrap();
    let file_inner: Box<dyn Layer<Registry> + Send + Sync> =
        Box::new(fmt::layer().compact().with_ansi(false).with_writer(file));
    let file_filtered = file_inner.with_filter(LevelFilter::INFO);
    let (file_layer, file_layer_reload) = reload::Layer::new(file_filtered);

    if get_console_log_enabled() {
        let console_layer = fmt::layer()
            .with_ansi(true)
            .with_filter(filter::LevelFilter::from_level(Level::INFO));

        Registry::default()
            .with(file_layer)
            .with(console_layer)
            .init();
    } else {
        Registry::default().with(file_layer).init();
    }

    Ok(file_layer_reload)
}
