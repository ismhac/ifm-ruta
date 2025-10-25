//! Process management interface

use std::path::Path;
use std::result::Result;
use std::time::Duration;

/// Process management interface
pub trait ProcessManager {
    /// Spawn a new process
    fn spawn_process(
        &self,
        command: &str,
        args: &[String],
        cwd: &Path,
    ) -> Result<ProcessHandle, ProcessError>;

    /// Kill a running process
    fn kill_process(&self, handle: &ProcessHandle) -> Result<(), ProcessError>;

    /// Wait for a process to complete
    fn wait_for_process(&self, handle: &ProcessHandle) -> Result<ProcessResult, ProcessError>;

    /// Get process output (stdout/stderr)
    fn get_process_output(&self, handle: &ProcessHandle) -> Result<ProcessOutput, ProcessError>;
}

/// Process handle for tracking spawned processes
#[derive(Debug, Clone)]
pub struct ProcessHandle {
    pub id: String,
    pub command: String,
    pub args: Vec<String>,
    pub cwd: std::path::PathBuf,
    pub status: ProcessStatus,
}

/// Process status
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessStatus {
    Running,
    Completed,
    Failed,
    Killed,
}

/// Process execution result
#[derive(Debug, Clone)]
pub struct ProcessResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration: Duration,
}

/// Process output
#[derive(Debug, Clone)]
pub struct ProcessOutput {
    pub stdout: String,
    pub stderr: String,
    pub is_complete: bool,
}

/// Process management error
#[derive(Debug, thiserror::Error)]
pub enum ProcessError {
    #[error("Process not found: {id}")]
    ProcessNotFound { id: String },

    #[error("Process already running: {id}")]
    ProcessAlreadyRunning { id: String },

    #[error("Process execution failed: {message}")]
    ExecutionFailed { message: String },

    #[error("Permission denied: {message}")]
    PermissionDenied { message: String },

    #[error("Timeout: process execution timed out")]
    Timeout,

    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error),
}
