//! Serialization utilities

use serde::{Deserialize, Serialize};
use std::result::Result;

/// Serialization format
#[derive(Debug, Clone, Copy)]
pub enum SerializationFormat {
    Json,
    Toml,
    Yaml,
}

/// Serialization manager
pub struct SerializationManager {
    format: SerializationFormat,
}

impl SerializationManager {
    /// Create a new serialization manager
    pub fn new(format: SerializationFormat) -> Self {
        Self { format }
    }
    
    /// Serialize data to bytes
    pub fn serialize<T>(&self, data: &T) -> Result<Vec<u8>, SerializationError>
    where
        T: Serialize,
    {
        match self.format {
            SerializationFormat::Json => {
                let json = serde_json::to_string_pretty(data)?;
                Ok(json.into_bytes())
            }
            SerializationFormat::Toml => {
                let toml = toml::to_string(data)?;
                Ok(toml.into_bytes())
            }
            SerializationFormat::Yaml => {
                let yaml = serde_yaml::to_string(data)?;
                Ok(yaml.into_bytes())
            }
        }
    }
    
    /// Deserialize data from bytes
    pub fn deserialize<T>(&self, data: &[u8]) -> Result<T, SerializationError>
    where
        T: for<'de> Deserialize<'de>,
    {
        match self.format {
            SerializationFormat::Json => {
                let json = String::from_utf8(data.to_vec())?;
                Ok(serde_json::from_str(&json)?)
            }
            SerializationFormat::Toml => {
                let toml = String::from_utf8(data.to_vec())?;
                Ok(toml::from_str(&toml)?)
            }
            SerializationFormat::Yaml => {
                let yaml = String::from_utf8(data.to_vec())?;
                Ok(serde_yaml::from_str(&yaml)?)
            }
        }
    }
}

/// Serialization error
#[derive(Debug, thiserror::Error)]
pub enum SerializationError {
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("TOML deserialize error: {0}")]
    TomlDeserializeError(#[from] toml::de::Error),
    
    #[error("TOML serialize error: {0}")]
    TomlSerializeError(#[from] toml::ser::Error),
    
    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),
    
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
