//! Settings manager implementation

use std::path::PathBuf;
use std::collections::HashMap;

use crate::traits::{SettingsManager, SettingsError};
use crate::models::{AppSettings, ProjectSettings};

/// Settings manager implementation
pub struct SettingsManagerImpl {
    settings: AppSettings,
    settings_path: PathBuf,
    project_settings: HashMap<PathBuf, ProjectSettings>,
}

impl SettingsManagerImpl {
    /// Create a new settings manager
    pub fn new() -> Self {
        let settings_path = Self::get_settings_path();
        Self {
            settings: AppSettings::default(),
            settings_path,
            project_settings: HashMap::new(),
        }
    }
    
    /// Get the settings file path
    fn get_settings_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ifm-ruta")
            .join("settings.toml")
    }
    
    /// Get project settings path
    fn get_project_settings_path(project_dir: &PathBuf) -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ifm-ruta")
            .join("projects")
            .join(format!("{}.toml", project_dir.file_name().unwrap_or_default().to_string_lossy()))
    }
}

impl SettingsManager for SettingsManagerImpl {
    fn load_settings(&mut self) -> Result<(), SettingsError> {
        if self.settings_path.exists() {
            let content = std::fs::read_to_string(&self.settings_path)?;
            self.settings = toml::from_str(&content)?;
        }
        Ok(())
    }
    
    fn save_settings(&self) -> Result<(), SettingsError> {
        // Create directory if it doesn't exist
        if let Some(parent) = self.settings_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(&self.settings)?;
        std::fs::write(&self.settings_path, content)?;
        Ok(())
    }
    
    fn reset_settings(&self) -> Result<(), SettingsError> {
        let default_settings = AppSettings::default();
        let content = toml::to_string_pretty(&default_settings)?;
        std::fs::write(&self.settings_path, content)?;
        Ok(())
    }
}