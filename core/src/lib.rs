//! Core system for Interactive Feedback MCP
//! 
//! This module provides the core interfaces, models, and services
//! that form the foundation of the Interactive Feedback MCP system.

pub mod traits;
pub mod models;
pub mod services;
pub mod utils;

// Re-export specific types to avoid conflicts
pub use models::{AppSettings, ProjectSettings, Feedback, AppError};
pub use traits::{Tool, Command, SettingsManager, ProcessManager, EventBus};
pub use services::{ConversationStorage, SettingsManagerImpl, ProcessManagerImpl, EventBusImpl};
pub use utils::{init_logging, error_handling};
