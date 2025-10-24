//! Interactive feedback tool implementation

use serde_json::{Value, json};
use std::process::Command;

use ifm_ruta_core::{
    traits::{Tool, ToolError, ValidationError},
};

/// Interactive feedback tool
pub struct InteractiveFeedbackTool;

impl InteractiveFeedbackTool {
    /// Create a new interactive feedback tool
    pub fn new() -> Self {
        Self
    }
    
    // Removed unused methods: get_conversation_history and create_feedback_with_history
    
    /// Create conversation history in Go format
    fn create_conversation_history_format(&self, previous_user_request: &str, prompt: &str) -> Result<Value, ToolError> {
        use uuid::Uuid;
        use chrono::Utc;
        
        let conversation_history = json!([
            {
                "id": Uuid::new_v4().to_string(),
                "timestamp": Utc::now().to_rfc3339(),
                "role": "user",
                "content": previous_user_request,
                "is_current": false
            },
            {
                "id": Uuid::new_v4().to_string(),
                "timestamp": Utc::now().to_rfc3339(),
                "role": "assistant", 
                "content": prompt,
                "is_current": false
            }
        ]);
        
        Ok(conversation_history)
    }
    
    /// Run interactive feedback with egui GUI (Rust native)
    fn run_interactive_feedback_with_gui(&self, project_directory: &str, prompt: &str) -> Result<String, ToolError> {
        // Find the egui binary (ifm-ruta-egui)
        let current_exe = std::env::current_exe()
            .map_err(|e| ToolError::ExecutionError {
                message: format!("Failed to get executable path: {}", e),
            })?;
        
        let exe_dir = current_exe.parent()
            .ok_or_else(|| ToolError::ExecutionError {
                message: "Failed to get executable directory".to_string(),
            })?;
        
        // Look for ifm-ruta-egui binary in the same directory
        let egui_binary = exe_dir.join("ifm-ruta-egui");
        let egui_path = if egui_binary.exists() {
            egui_binary
        } else {
            // Try alternative path (parent directory)
            exe_dir.parent()
                .ok_or_else(|| ToolError::ExecutionError {
                    message: "Failed to get parent directory".to_string(),
                })?
                .join("ifm-ruta-egui")
        };
        
        if !egui_path.exists() {
            return Err(ToolError::ExecutionError {
                message: "egui binary not found. Please ensure ifm-ruta-egui is built and in the project directory.".to_string(),
            });
        }
        
        // Run egui GUI with project directory and prompt as arguments
        let output = Command::new(egui_path.to_str().unwrap())
            .arg(project_directory)
            .arg(prompt)
            .output()
            .map_err(|e| ToolError::ExecutionError {
                message: format!("Failed to run egui GUI: {}", e),
            })?;
        
        if !output.status.success() {
            return Err(ToolError::ExecutionError {
                message: format!("egui GUI failed: {}", String::from_utf8_lossy(&output.stderr)),
            });
        }
        
        let user_feedback = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(user_feedback)
    }
    
    /// Save real conversation to storage
    fn save_real_conversation(&self, project_directory: &str, previous_user_request: &str, prompt: &str) -> Result<(), ToolError> {
        use std::path::Path;
        use ifm_ruta_core::services::ConversationStorage;
        use ifm_ruta_core::ConversationSession;
        use ifm_ruta_core::ConversationMessage;
        use uuid::Uuid;
        use chrono::Utc;
        
        // Setup project directory with .gitignore and README
        self.setup_project_directory(project_directory)?;
        
        let storage = ConversationStorage::new(Path::new(project_directory));
        storage.initialize()
            .map_err(|e| ToolError::ExecutionError {
                message: format!("Failed to initialize storage: {}", e),
            })?;
        
        // Create conversation session
        let session_id = format!("cursor-chat-{}", Uuid::new_v4().to_string()[..8].to_string());
        let now = Utc::now().to_rfc3339();
        
        let mut messages = Vec::new();
        
        // Add user message
        if !previous_user_request.is_empty() {
            messages.push(ConversationMessage {
                role: "user".to_string(),
                content: previous_user_request.to_string(),
                timestamp: now.clone(),
            });
        }
        
        // Add assistant message
        messages.push(ConversationMessage {
            role: "assistant".to_string(),
            content: prompt.to_string(),
            timestamp: now.clone(),
        });
        
        let session = ConversationSession {
            session_id: session_id.clone(),
            project_directory: Path::new(project_directory).to_path_buf(),
            messages,
            created_at: now.clone(),
            last_updated: now,
        };
        
        storage.save_session(&session)
            .map_err(|e| ToolError::ExecutionError {
                message: format!("Failed to save conversation: {}", e),
            })?;
        
        println!("Saved real conversation to storage: {}", session_id);
        Ok(())
    }
    
    /// Setup project directory with .gitignore and README
    fn setup_project_directory(&self, project_directory: &str) -> Result<(), ToolError> {
        use std::path::Path;
        use std::fs;
        
        let project_path = Path::new(project_directory);
        
        // Create .ifm-ruta directory if it doesn't exist
        let ifm_ruta_dir = project_path.join(".ifm-ruta");
        if !ifm_ruta_dir.exists() {
            fs::create_dir_all(&ifm_ruta_dir)
                .map_err(|e| ToolError::ExecutionError {
                    message: format!("Failed to create .ifm-ruta directory: {}", e),
                })?;
            println!("Created .ifm-ruta directory");
        }
        
        // Create README.md in .ifm-ruta directory
        let readme_path = ifm_ruta_dir.join("README.md");
        if !readme_path.exists() {
            let readme_content = "# IFM-Ruta Conversation History

This directory contains conversation history between users and the IFM-Ruta MCP (Model Context Protocol) assistant.

## Directory Structure

- `conversations/` - Contains JSON files with conversation sessions
- `README.md` - This file explaining the directory purpose

## Files

- Each conversation session is stored as a separate JSON file
- Files are named with the session ID (e.g., `cursor-chat-abc123.json`)
- Each file contains the complete conversation history for that session

## Privacy

This directory contains conversation data that may include sensitive information.
It is automatically added to `.gitignore` to prevent accidental commits.

## Generated by IFM-Ruta MCP

This directory and its contents are automatically managed by the IFM-Ruta MCP tool.
Do not manually modify the files in this directory.
";
            
            fs::write(&readme_path, readme_content)
                .map_err(|e| ToolError::ExecutionError {
                    message: format!("Failed to create README.md: {}", e),
                })?;
            println!("Created README.md in .ifm-ruta directory");
        }
        
        // Setup .gitignore
        self.setup_gitignore(project_directory)?;
        
        Ok(())
    }
    
    /// Setup .gitignore to exclude .ifm-ruta directory
    fn setup_gitignore(&self, project_directory: &str) -> Result<(), ToolError> {
        use std::path::Path;
        use std::fs;
        
        let gitignore_path = Path::new(project_directory).join(".gitignore");
        let ifm_ruta_ignore = ".ifm-ruta/\n";
        
        if gitignore_path.exists() {
            // Read existing .gitignore
            let existing_content = fs::read_to_string(&gitignore_path)
                .map_err(|e| ToolError::ExecutionError {
                    message: format!("Failed to read .gitignore: {}", e),
                })?;
            
            // Check if .ifm-ruta is already ignored
            if !existing_content.contains(".ifm-ruta/") {
                // Append .ifm-ruta to existing .gitignore
                let updated_content = format!("{}\n{}", existing_content.trim_end(), ifm_ruta_ignore);
                fs::write(&gitignore_path, updated_content)
                    .map_err(|e| ToolError::ExecutionError {
                        message: format!("Failed to update .gitignore: {}", e),
                    })?;
                println!("Added .ifm-ruta/ to existing .gitignore");
            } else {
                println!(".ifm-ruta/ already in .gitignore");
            }
        } else {
            // Create new .gitignore
            fs::write(&gitignore_path, ifm_ruta_ignore)
                .map_err(|e| ToolError::ExecutionError {
                    message: format!("Failed to create .gitignore: {}", e),
                })?;
            println!("Created .gitignore with .ifm-ruta/ entry");
        }
        
        Ok(())
    }
}

impl Tool for InteractiveFeedbackTool {
    fn name(&self) -> &str {
        "interactive_feedback"
    }
    
    fn description(&self) -> &str {
        "Get interactive feedback from user for development tasks"
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "projectDirectory": {
                    "type": "string",
                    "description": "The project directory path"
                },
                "prompt": {
                    "type": "string",
                    "description": "The prompt to show to the user"
                },
                "previousUserRequest": {
                    "type": "string",
                    "description": "The previous user request that triggered this interactive feedback"
                }
            },
            "required": ["projectDirectory", "prompt", "previousUserRequest"]
        })
    }
    
    fn execute(&self, input: Value) -> Result<Value, ToolError> {
        // Validate input
        self.validate_input(&input)?;
        
        // Extract parameters (matching Go schema)
        let project_directory = input.get("projectDirectory")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingField {
                field: "projectDirectory".to_string(),
            })?;
        
        let prompt = input.get("prompt")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingField {
                field: "prompt".to_string(),
            })?;
        
        let previous_user_request = input.get("previousUserRequest")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingField {
                field: "previousUserRequest".to_string(),
            })?;
        
        // Save real conversation to storage
        self.save_real_conversation(project_directory, previous_user_request, prompt)?;
        
        // Run interactive feedback with Python GUI like Go implementation
        let user_feedback = self.run_interactive_feedback_with_gui(project_directory, prompt)?;
        
        // Create conversation history in Go format
        let conversation_history = self.create_conversation_history_format(previous_user_request, prompt)?;
        
        // Return the result in Go format
        Ok(json!({
            "command_logs": "",
            "interactive_feedback": user_feedback,
            "conversation_history": conversation_history
        }))
    }
    
    fn validate_input(&self, input: &Value) -> Result<(), ValidationError> {
        // Check required fields
        if !input.is_object() {
            return Err(ValidationError::InvalidInput {
                message: "Input must be an object".to_string(),
            });
        }
        
        let obj = input.as_object().unwrap();
        
        // Check projectDirectory
        if !obj.contains_key("projectDirectory") {
            return Err(ValidationError::MissingField {
                field: "projectDirectory".to_string(),
            });
        }
        
        if let Some(project_dir) = obj.get("projectDirectory") {
            if !project_dir.is_string() {
                return Err(ValidationError::InvalidType {
                    field: "projectDirectory".to_string(),
                    expected: "string".to_string(),
                });
            }
        }
        
        // Check prompt
        if !obj.contains_key("prompt") {
            return Err(ValidationError::MissingField {
                field: "prompt".to_string(),
            });
        }
        
        if let Some(prompt) = obj.get("prompt") {
            if !prompt.is_string() {
                return Err(ValidationError::InvalidType {
                    field: "prompt".to_string(),
                    expected: "string".to_string(),
                });
            }
        }
        
        // Check previousUserRequest
        if !obj.contains_key("previousUserRequest") {
            return Err(ValidationError::MissingField {
                field: "previousUserRequest".to_string(),
            });
        }
        
        if let Some(previous_request) = obj.get("previousUserRequest") {
            if !previous_request.is_string() {
                return Err(ValidationError::InvalidType {
                    field: "previousUserRequest".to_string(),
                    expected: "string".to_string(),
                });
            }
        }
        
        Ok(())
    }
}
