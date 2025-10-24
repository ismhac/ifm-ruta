//! Conversation storage service for managing user-agent conversation history

use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use crate::models::AppError;

/// Conversation message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub role: String, // "user" or "assistant"
    pub content: String,
    pub timestamp: String, // Use string for easier serialization
}

/// Conversation session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSession {
    pub session_id: String,
    pub project_directory: PathBuf,
    pub messages: Vec<ConversationMessage>,
    pub created_at: String, // Use string for easier serialization
    pub last_updated: String, // Use string for easier serialization
}

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_sessions: usize,
    pub total_messages: usize,
    pub total_size_bytes: u64,
}

/// Conversation storage service
pub struct ConversationStorage {
    storage_dir: PathBuf,
}

impl ConversationStorage {
    /// Create a new conversation storage service
    pub fn new(project_directory: &Path) -> Self {
        let storage_dir = project_directory.join(".ifm-ruta").join("conversations");
        Self { storage_dir }
    }
    
    /// Initialize storage directory
    pub fn initialize(&self) -> Result<(), AppError> {
        if !self.storage_dir.exists() {
            fs::create_dir_all(&self.storage_dir)
                .map_err(|e| AppError::StorageError {
                    message: format!("Failed to create storage directory: {}", e),
                })?;
        }
        Ok(())
    }
    
    /// Save a conversation session
    pub fn save_session(&self, session: &ConversationSession) -> Result<(), AppError> {
        self.initialize()?;
        
        let session_file = self.storage_dir.join(format!("{}.json", session.session_id));
        let content = serde_json::to_string_pretty(session)
            .map_err(|e| AppError::SerializationError {
                message: format!("Failed to serialize session: {}", e),
            })?;
        
        fs::write(&session_file, content)
            .map_err(|e| AppError::StorageError {
                message: format!("Failed to write session file: {}", e),
            })?;
        
        Ok(())
    }
    
    /// Load a conversation session
    pub fn load_session(&self, session_id: &str) -> Result<Option<ConversationSession>, AppError> {
        let session_file = self.storage_dir.join(format!("{}.json", session_id));
        
        if !session_file.exists() {
            return Ok(None);
        }
        
        let content = fs::read_to_string(&session_file)
            .map_err(|e| AppError::StorageError {
                message: format!("Failed to read session file: {}", e),
            })?;
        
        let session: ConversationSession = serde_json::from_str(&content)
            .map_err(|e| AppError::DeserializationError {
                message: format!("Failed to deserialize session: {}", e),
            })?;
        
        Ok(Some(session))
    }
    
    /// Get all conversation sessions for a project
    pub fn get_project_sessions(&self) -> Result<Vec<ConversationSession>, AppError> {
        self.initialize()?;
        
        let mut sessions = Vec::new();
        
        println!("Looking for conversations in: {:?}", self.storage_dir);
        
        if !self.storage_dir.exists() {
            println!("Storage directory does not exist: {:?}", self.storage_dir);
            return Ok(sessions);
        }
        
        let entries = fs::read_dir(&self.storage_dir)
            .map_err(|e| AppError::StorageError {
                message: format!("Failed to read storage directory: {}", e),
            })?;
        
        for entry in entries {
            let entry = entry.map_err(|e| AppError::StorageError {
                message: format!("Failed to read directory entry: {}", e),
            })?;
            
            println!("Found file: {:?}", entry.path());
            
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                let path = entry.path();
                let session_id = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                println!("Loading session: {}", session_id);
                
                if let Ok(session) = self.load_session(session_id) {
                    if let Some(session) = session {
                        println!("Loaded session with {} messages", session.messages.len());
                        sessions.push(session);
                    }
                } else {
                    println!("Failed to load session: {}", session_id);
                }
            }
        }
        
        // Sort by last updated (newest first)
        sessions.sort_by(|a, b| b.last_updated.cmp(&a.last_updated));
        
        Ok(sessions)
    }
    
    /// Get latest 5 conversation sessions (keep only 5 most recent)
    pub fn get_latest_5_sessions(&self) -> Result<Vec<ConversationSession>, AppError> {
        let mut sessions = self.get_project_sessions()?;
        
        // Keep only the latest 5 sessions
        if sessions.len() > 5 {
            // Delete older sessions beyond the 5 most recent
            let sessions_to_delete = sessions.split_off(5);
            
            for session in sessions_to_delete {
                let session_file = self.storage_dir.join(format!("{}.json", session.session_id));
                if session_file.exists() {
                    let _ = fs::remove_file(&session_file);
                }
            }
        }
        
        Ok(sessions)
    }
    
    /// Add a message to a conversation session
    pub fn add_message(&self, session_id: &str, role: &str, content: &str) -> Result<(), AppError> {
        let mut session = self.load_session(session_id)?
            .unwrap_or_else(|| ConversationSession {
                session_id: session_id.to_string(),
                project_directory: self.storage_dir.parent().unwrap().to_path_buf(),
                messages: Vec::new(),
                created_at: chrono::Utc::now().to_rfc3339(),
                last_updated: chrono::Utc::now().to_rfc3339(),
            });
        
        let message = ConversationMessage {
            role: role.to_string(),
            content: content.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        
        session.messages.push(message);
        session.last_updated = chrono::Utc::now().to_rfc3339();
        
        self.save_session(&session)?;
        Ok(())
    }
    
    /// Get conversation history as formatted string
    pub fn get_conversation_history(&self, session_id: &str) -> Result<String, AppError> {
        let session = self.load_session(session_id)?;
        
        match session {
            Some(session) => {
                let mut history = String::new();
                history.push_str(&format!("Conversation Session: {}\n", session.session_id));
                history.push_str(&format!("Project: {}\n", session.project_directory.display()));
                history.push_str(&format!("Created: {:?}\n", session.created_at));
                history.push_str(&format!("Last Updated: {:?}\n\n", session.last_updated));
                
                for (i, message) in session.messages.iter().enumerate() {
                    history.push_str(&format!("Message {}:\n", i + 1));
                    history.push_str(&format!("Role: {}\n", message.role));
                    history.push_str(&format!("Timestamp: {:?}\n", message.timestamp));
                    history.push_str(&format!("Content: {}\n\n", message.content));
                }
                
                Ok(history)
            }
            None => Ok("No conversation history found for this session.".to_string()),
        }
    }
    
    /// Get conversation history from latest 5 sessions
    pub fn get_latest_5_conversations(&self) -> Result<String, AppError> {
        let sessions = self.get_latest_5_sessions()?;
        
        if sessions.is_empty() {
            return Ok("No conversation history found for this project.".to_string());
        }
        
        let mut history = String::new();
        history.push_str("=== LATEST 5 CONVERSATIONS ===\n\n");
        
        for (session_index, session) in sessions.iter().enumerate() {
            history.push_str(&format!("--- SESSION {} ---\n", session_index + 1));
            history.push_str(&format!("Session ID: {}\n", session.session_id));
            history.push_str(&format!("Created: {:?}\n", session.created_at));
            history.push_str(&format!("Last Updated: {:?}\n", session.last_updated));
            history.push_str(&format!("Messages: {}\n\n", session.messages.len()));
            
            for (i, message) in session.messages.iter().enumerate() {
                history.push_str(&format!("  Message {}:\n", i + 1));
                history.push_str(&format!("  Role: {}\n", message.role));
                history.push_str(&format!("  Content: {}\n\n", message.content));
            }
            
            history.push_str("\n");
        }
        
        Ok(history)
    }
    
    /// Get the latest conversation session
    pub fn get_latest_session(&self) -> Result<Option<ConversationSession>, AppError> {
        let sessions = self.get_project_sessions()?;
        Ok(sessions.into_iter().next())
    }
    
    /// Get storage statistics
    pub fn get_storage_stats(&self) -> Result<StorageStats, AppError> {
        let sessions = self.get_project_sessions()?;
        let total_sessions = sessions.len();
        let total_messages: usize = sessions.iter().map(|s| s.messages.len()).sum();
        
        // Calculate total storage size
        let mut total_size = 0;
        for session in &sessions {
            let session_file = self.storage_dir.join(format!("{}.json", session.session_id));
            if let Ok(metadata) = session_file.metadata() {
                total_size += metadata.len();
            }
        }
        
        Ok(StorageStats {
            total_sessions,
            total_messages,
            total_size_bytes: total_size,
        })
    }
    
    /// Clean up old sessions (keep only the most recent N sessions)
    pub fn cleanup_old_sessions(&self, keep_count: usize) -> Result<usize, AppError> {
        let mut sessions = self.get_project_sessions()?;
        
        if sessions.len() <= keep_count {
            return Ok(0);
        }
        
        // Sort by creation time (oldest first for deletion)
        sessions.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        
        let to_delete = sessions.len() - keep_count;
        let mut deleted_count = 0;
        
        for session in sessions.iter().take(to_delete) {
            let session_file = self.storage_dir.join(format!("{}.json", session.session_id));
            if session_file.exists() {
                fs::remove_file(&session_file)
                    .map_err(|e| AppError::StorageError {
                        message: format!("Failed to delete old session file: {}", e),
                    })?;
                deleted_count += 1;
                println!("Deleted old session: {}", session.session_id);
            }
        }
        
        Ok(deleted_count)
    }
}
