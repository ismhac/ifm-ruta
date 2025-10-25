//! Conversation logging utility for tracking user-agent interactions

use crate::models::AppError;
use crate::services::ConversationStorage;
use std::path::Path;
use uuid::Uuid;

/// Conversation logger for tracking user-agent interactions
pub struct ConversationLogger {
    storage: ConversationStorage,
    current_session_id: String,
}

impl ConversationLogger {
    /// Create a new conversation logger
    pub fn new(project_directory: &Path) -> Self {
        let storage = ConversationStorage::new(project_directory);
        let current_session_id = Uuid::new_v4().to_string();

        Self {
            storage,
            current_session_id,
        }
    }

    /// Initialize the logger
    pub fn initialize(&self) -> Result<(), AppError> {
        self.storage.initialize()
    }

    /// Log a user message
    pub fn log_user_message(&self, content: &str) -> Result<(), AppError> {
        self.storage
            .add_message(&self.current_session_id, "user", content)
    }

    /// Log an assistant message
    pub fn log_assistant_message(&self, content: &str) -> Result<(), AppError> {
        self.storage
            .add_message(&self.current_session_id, "assistant", content)
    }

    /// Get current session ID
    pub fn get_session_id(&self) -> &str {
        &self.current_session_id
    }

    /// Create a new session
    pub fn new_session(&mut self) {
        self.current_session_id = Uuid::new_v4().to_string();
    }

    /// Get conversation history (latest 5)
    pub fn get_conversation_history(&self) -> Result<String, AppError> {
        self.storage.get_latest_5_conversations()
    }
}

/// Global conversation logger instance
static mut CONVERSATION_LOGGER: Option<ConversationLogger> = None;

/// Initialize global conversation logger
pub fn init_conversation_logger(project_directory: &Path) -> Result<(), AppError> {
    unsafe {
        CONVERSATION_LOGGER = Some(ConversationLogger::new(project_directory));
        if let Some(ref logger) = CONVERSATION_LOGGER {
            logger.initialize()?;
        }
    }
    Ok(())
}

/// Log a user message globally
pub fn log_user_message(content: &str) -> Result<(), AppError> {
    unsafe {
        if let Some(ref logger) = CONVERSATION_LOGGER {
            logger.log_user_message(content)?;
        }
    }
    Ok(())
}

/// Log an assistant message globally
pub fn log_assistant_message(content: &str) -> Result<(), AppError> {
    unsafe {
        if let Some(ref logger) = CONVERSATION_LOGGER {
            logger.log_assistant_message(content)?;
        }
    }
    Ok(())
}

/// Get conversation history globally
pub fn get_conversation_history() -> Result<String, AppError> {
    unsafe {
        if let Some(ref logger) = CONVERSATION_LOGGER {
            logger.get_conversation_history()
        } else {
            Ok("Conversation logger not initialized.".to_string())
        }
    }
}
