//! Logging utilities

use tracing::Level;

/// Initialize logging
pub fn init_logging(level: Level) -> Result<(), LoggingError> {
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    Ok(())
}

/// Logging error
#[derive(Debug, thiserror::Error)]
pub enum LoggingError {
    #[error("Failed to initialize logging: {message}")]
    InitializationError { message: String },

    #[error("Logging configuration error: {message}")]
    ConfigurationError { message: String },
}
