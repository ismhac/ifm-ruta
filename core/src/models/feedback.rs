//! Feedback model and related types

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

/// Feedback data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    pub project_directory: PathBuf,
    pub summary: String,
    pub command_logs: String,
    pub interactive_feedback: String,
    pub conversation_history: String,
    pub timestamp: SystemTime,
    pub metadata: FeedbackMetadata,
}

/// Feedback metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackMetadata {
    pub tool_name: String,
    pub execution_time: Duration,
    pub commands_executed: Vec<String>,
    pub user_interaction_time: Duration,
}

impl Default for FeedbackMetadata {
    fn default() -> Self {
        Self {
            tool_name: "interactive_feedback".to_string(),
            execution_time: Duration::from_secs(0),
            commands_executed: vec![],
            user_interaction_time: Duration::from_secs(0),
        }
    }
}

impl Feedback {
    /// Create a new feedback instance
    pub fn new(project_directory: PathBuf, summary: String) -> Self {
        Self {
            project_directory,
            summary,
            command_logs: String::new(),
            interactive_feedback: String::new(),
            conversation_history: String::new(),
            timestamp: SystemTime::now(),
            metadata: FeedbackMetadata::default(),
        }
    }

    /// Set command logs
    pub fn set_command_logs(&mut self, logs: String) {
        self.command_logs = logs;
    }

    /// Set interactive feedback
    pub fn set_interactive_feedback(&mut self, feedback: String) {
        self.interactive_feedback = feedback;
    }

    /// Add executed command
    pub fn add_executed_command(&mut self, command: String) {
        self.metadata.commands_executed.push(command);
    }

    /// Set execution time
    pub fn set_execution_time(&mut self, duration: Duration) {
        self.metadata.execution_time = duration;
    }

    /// Set user interaction time
    pub fn set_user_interaction_time(&mut self, duration: Duration) {
        self.metadata.user_interaction_time = duration;
    }

    /// Set conversation history
    pub fn set_conversation_history(&mut self, history: String) {
        self.conversation_history = history;
    }
}
