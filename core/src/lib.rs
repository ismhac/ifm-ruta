//! Core system for Interactive Feedback MCP
//!
//! This module provides the core interfaces, models, and services
//! that form the foundation of the Interactive Feedback MCP system.

pub mod models;
pub mod services;
pub mod traits;
pub mod utils;

// Re-export specific types to avoid conflicts
pub use models::{AppError, AppSettings, Feedback, ProjectSettings};
pub use services::{ConversationStorage, EventBusImpl, ProcessManagerImpl, SettingsManagerImpl};
pub use traits::{Command, EventBus, ProcessManager, SettingsManager, Tool};
pub use utils::{error_handling, init_logging};
