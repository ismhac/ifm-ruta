//! Interactive feedback tool implementation

use std::path::PathBuf;
use serde_json::{Value, json};
use std::process::Command;

use ifm_ruta_core::{
    traits::{Tool, ToolError, ValidationError},
    models::{Feedback, AppError},
};

/// Interactive feedback tool
pub struct InteractiveFeedbackTool;

impl InteractiveFeedbackTool {
    /// Create a new interactive feedback tool
    pub fn new() -> Self {
        Self
    }
    
    /// Launch the Tauri application
    fn launch_tauri_app(&self, project_directory: &str, summary: &str) -> Result<Feedback, ToolError> {
        // Create feedback instance
        let mut feedback = Feedback::new(
            PathBuf::from(project_directory),
            summary.to_string(),
        );
        
        // Launch Tauri app (simplified implementation)
        let tauri_path = std::env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(|p| p.join("ifm-ruta-tauri")))
            .unwrap_or_else(|| std::path::PathBuf::from("./dist/ifm-ruta-tauri"));
        
        let output = Command::new(&tauri_path)
            .arg("--project-directory")
            .arg(project_directory)
            .arg("--summary")
            .arg(summary)
            .output()
            .map_err(|e| ToolError::ExecutionError {
                message: format!("Failed to launch Tauri app: {}", e),
            })?;
        
        if !output.status.success() {
            return Err(ToolError::ExecutionError {
                message: "Tauri app failed to execute".to_string(),
            });
        }
        
        // Parse the output (simplified)
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // Parse JSON response (simplified)
        if let Ok(result) = serde_json::from_str::<Value>(&output_str) {
            if let Some(command_logs) = result.get("command_logs").and_then(|v| v.as_str()) {
                feedback.set_command_logs(command_logs.to_string());
            }
            if let Some(interactive_feedback) = result.get("interactive_feedback").and_then(|v| v.as_str()) {
                feedback.set_interactive_feedback(interactive_feedback.to_string());
            }
        }
        
        Ok(feedback)
    }
}

impl Tool for InteractiveFeedbackTool {
    fn name(&self) -> &str {
        "interactive_feedback"
    }
    
    fn description(&self) -> &str {
        "Request interactive feedback for a given project directory and summary"
    }
    
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "project_directory": {
                    "type": "string",
                    "description": "Full path to the project directory"
                },
                "summary": {
                    "type": "string",
                    "description": "Short, one-line summary of the changes"
                }
            },
            "required": ["project_directory", "summary"]
        })
    }
    
    fn execute(&self, input: Value) -> Result<Value, ToolError> {
        // Validate input
        self.validate_input(&input)?;
        
        // Extract parameters
        let project_directory = input.get("project_directory")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingField {
                field: "project_directory".to_string(),
            })?;
        
        let summary = input.get("summary")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingField {
                field: "summary".to_string(),
            })?;
        
        // Launch Tauri app and get feedback
        let feedback = self.launch_tauri_app(project_directory, summary)?;
        
        // Return the result
        Ok(json!({
            "command_logs": feedback.command_logs,
            "interactive_feedback": feedback.interactive_feedback
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
        
        // Check project_directory
        if !obj.contains_key("project_directory") {
            return Err(ValidationError::MissingField {
                field: "project_directory".to_string(),
            });
        }
        
        if let Some(project_dir) = obj.get("project_directory") {
            if !project_dir.is_string() {
                return Err(ValidationError::InvalidType {
                    field: "project_directory".to_string(),
                    expected: "string".to_string(),
                });
            }
        }
        
        // Check summary
        if !obj.contains_key("summary") {
            return Err(ValidationError::MissingField {
                field: "summary".to_string(),
            });
        }
        
        if let Some(summary) = obj.get("summary") {
            if !summary.is_string() {
                return Err(ValidationError::InvalidType {
                    field: "summary".to_string(),
                    expected: "string".to_string(),
                });
            }
        }
        
        Ok(())
    }
}
