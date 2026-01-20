pub fn setup_logging() {
    #[cfg(feature = "server")]
    {
        use crate::backend::env::get_logging_path;
        use std::fs::OpenOptions;
        use tracing::Level;
        use tracing_subscriber::{filter, fmt, layer::Layer, prelude::*, Registry};

        let mut log_file_path = get_logging_path();
        log_file_path.push_str("/logfile.log");

        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_file_path)
            .unwrap();

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
                    .with_writer(file)
                    .with_filter(filter::LevelFilter::from_level(Level::INFO)),
            );

        tracing::subscriber::set_global_default(subscriber).unwrap();
    }
}
