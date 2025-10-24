//! Settings management interface

use std::result::Result;

/// Settings management interface
pub trait SettingsManager: Send + Sync {
    /// Load settings from storage
    fn load_settings(&mut self) -> Result<(), SettingsError>;
    
    /// Save settings to storage
    fn save_settings(&self) -> Result<(), SettingsError>;
    
    /// Reset settings to defaults
    fn reset_settings(&self) -> Result<(), SettingsError>;
}

/// Settings error types
#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Permission denied: {message}")]
    PermissionDenied { message: String },
    
    #[error("Invalid format: {message}")]
    InvalidFormat { message: String },
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error),
    
    #[error("TOML deserialization error: {0}")]
    TomlDeserializeError(#[from] toml::de::Error),
    
    #[error("TOML serialization error: {0}")]
    TomlSerializeError(#[from] toml::ser::Error),
}
