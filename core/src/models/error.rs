//! Core error types

use thiserror::Error;

/// Core application error
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Tool error: {0}")]
    ToolError(#[from] crate::traits::ToolError),

    #[error("Command error: {0}")]
    CommandError(#[from] crate::traits::CommandError),

    #[error("Settings error: {0}")]
    SettingsError(#[from] crate::traits::SettingsError),

    #[error("Process error: {0}")]
    ProcessError(#[from] crate::traits::ProcessError),

    #[error("Event error: {0}")]
    EventError(#[from] crate::traits::EventError),

    #[error("Validation error: {0}")]
    ValidationError(#[from] crate::traits::ValidationError),

    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Logging error: {0}")]
    LoggingError(#[from] crate::utils::LoggingError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Storage error: {message}")]
    StorageError { message: String },

    #[error("Serialization error: {message}")]
    SerializationError { message: String },

    #[error("Deserialization error: {message}")]
    DeserializationError { message: String },
}

/// Result type for core operations
pub type Result<T> = std::result::Result<T, AppError>;
