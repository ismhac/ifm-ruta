//! Project model and related types

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

/// Project representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub directory: PathBuf,
    pub name: String,
    pub settings: ProjectSettings,
    pub metadata: ProjectMetadata,
}

/// Project-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub auto_execute: bool,
    pub default_command: Option<String>,
    pub ui_state: UIState,
}

/// UI state for the project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIState {
    pub window_size: (u32, u32),
    pub window_position: (i32, i32),
    pub show_command_section: bool,
}

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub created_at: SystemTime,
    pub last_accessed: SystemTime,
    pub version: String,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            auto_execute: false,
            default_command: None,
            ui_state: UIState {
                window_size: (800, 600),
                window_position: (100, 100),
                show_command_section: false,
            },
        }
    }
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            created_at: SystemTime::now(),
            last_accessed: SystemTime::now(),
            version: "1.0.0".to_string(),
        }
    }
}
