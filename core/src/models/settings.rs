//! Application settings model

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub general: GeneralSettings,
    pub ui: UISettings,
    pub security: SecuritySettings,
    pub performance: PerformanceSettings,
}

/// General application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub log_level: LogLevel,
    pub timeout: Duration,
    pub auto_save: bool,
}

/// UI settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISettings {
    pub theme: Theme,
    pub window_size: (u32, u32),
    pub window_position: (i32, i32),
    pub show_command_section: bool,
}

/// Security settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub allowed_commands: Vec<String>,
    pub sandbox_mode: bool,
    pub max_process_time: Duration,
}

/// Performance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    pub max_memory_usage: usize,
    pub log_rotation_size: usize,
    pub cache_size: usize,
}

/// Log level enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Theme enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            general: GeneralSettings {
                log_level: LogLevel::Info,
                timeout: Duration::from_secs(300),
                auto_save: true,
            },
            ui: UISettings {
                theme: Theme::Auto,
                window_size: (800, 600),
                window_position: (100, 100),
                show_command_section: false,
            },
            security: SecuritySettings {
                allowed_commands: vec![],
                sandbox_mode: true,
                max_process_time: Duration::from_secs(60),
            },
            performance: PerformanceSettings {
                max_memory_usage: 100 * 1024 * 1024, // 100MB
                log_rotation_size: 10 * 1024 * 1024, // 10MB
                cache_size: 50 * 1024 * 1024,        // 50MB
            },
        }
    }
}
