//! Command interface and related types

use serde_json::Value;
use std::path::PathBuf;
use std::result::Result;
use std::sync::Arc;

use crate::traits::{EventBus, ProcessManager, SettingsManager};

/// Core command interface
pub trait Command {
    /// Get the command name
    fn name(&self) -> &str;

    /// Get the command description
    fn description(&self) -> &str;

    /// Execute the command with given context
    fn execute(&self, context: &CommandContext) -> Result<CommandResult, CommandError>;

    /// Check if the command can be undone
    fn can_undo(&self) -> bool;

    /// Undo the command if possible
    fn undo(&self, context: &CommandContext) -> Result<(), CommandError>;
}

/// Command execution context
pub struct CommandContext {
    pub project_directory: PathBuf,
    pub settings: Arc<dyn SettingsManager>,
    pub process_manager: Arc<dyn ProcessManager>,
    pub event_bus: Arc<dyn EventBus>,
}

/// Command execution result
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub metadata: Value,
}

/// Command execution error
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Command not found: {name}")]
    CommandNotFound { name: String },

    #[error("Execution failed: {message}")]
    ExecutionFailed { message: String },

    #[error("Invalid arguments: {message}")]
    InvalidArguments { message: String },

    #[error("Permission denied: {message}")]
    PermissionDenied { message: String },

    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error),
}
