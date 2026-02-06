use crate::backend::env::{get_console_log_enabled, get_logging_file, get_logging_path};
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::sync::LazyLock;
use tracing::{error, level_filters::LevelFilter};
use tracing_subscriber::{
    filter::Filtered,
    fmt,
    layer::Layer,
    prelude::*,
    reload::{self, Handle},
    Registry,
};

pub type ReloadableLayer =
    Handle<Filtered<Box<dyn Layer<Registry> + Send + Sync>, LevelFilter, Registry>, Registry>;

pub static LOG_LAYERS: LazyLock<
    Result<
        (
            ReloadableLayer,         // File handle
            Option<ReloadableLayer>, // Optional console handle
        ),
        String,
    >,
> = LazyLock::new(setup_logging);

#[allow(dead_code)] // FIXME allow user/config to change at runtime
pub fn change_file_logger_level(level: LevelFilter) -> bool {
    match LOG_LAYERS.as_ref() {
        Ok((log_file_layer, _)) => match log_file_layer.modify(|layer| *layer.filter_mut() = level)
        {
            Ok(_) => true,
            Err(e) => {
                error!("Failed to update file logger level: {}", e);
                false
            }
        },
        Err(e) => {
            error!("Cannot update level of uninitialised logger: {}", e);
            false
        }
    }
}

#[allow(dead_code)] // FIXME allow user/config to change at runtime
pub fn change_console_logger_level(level: LevelFilter) -> bool {
    match LOG_LAYERS.as_ref() {
        Ok((_, console)) => match console {
            Some(console) => match console.modify(|layer| *layer.filter_mut() = level) {
                Ok(_) => true,
                Err(e) => {
                    error!("Failed to update console logger level: {}", e);
                    false
                }
            },
            None => {
                error!("Cannot update level of disabled console logger");
                false
            }
        },
        Err(e) => {
            error!("Cannot update level of uninitialised logger: {}", e);
            false
        }
    }
}

fn setup_logging() -> Result<(ReloadableLayer, Option<ReloadableLayer>), String> {
    let log_file_path = get_logging_path();
    let log_file = get_logging_file();

    if let Err(e) = std::fs::remove_file(&log_file) {
        if e.kind() != std::io::ErrorKind::NotFound {
            return Err(format!(
                "Unexpected error with log file: {}, with error: {}",
                log_file, e
            ));
        }
    }
    if let Err(e) = std::fs::create_dir_all(&log_file_path) {
        if e.kind() != ErrorKind::AlreadyExists {
            return Err(format!(
                "Failed to create logging dir: {} with error: {}",
                log_file_path, e
            ));
        }
    }

    let file = match OpenOptions::new().create(true).append(true).open(&log_file) {
        Ok(f) => f,
        Err(e) => {
            return Err(format!(
                "Failed to create log file: {} with error: {}",
                log_file, e
            ))
        }
    };

    let file_inner = fmt::layer().with_ansi(false).with_writer(file).boxed();
    let file_filtered = file_inner.with_filter(LevelFilter::INFO);
    let (file_layer, file_handle) = reload::Layer::new(file_filtered);

    if get_console_log_enabled() {
        let stdout_inner = fmt::layer().compact().with_ansi(true).boxed();
        let stdout_filtered = stdout_inner.with_filter(LevelFilter::INFO);
        let (stdout_layer, stdout_handle) = reload::Layer::new(stdout_filtered);

        Registry::default()
            .with(vec![file_layer.boxed(), stdout_layer.boxed()])
            .init();

        Ok((file_handle, Some(stdout_handle)))
    } else {
        Registry::default().with(file_layer).init();
        Ok((file_handle, None))
    }
}
