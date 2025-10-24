//! Tool interface and related types

use serde_json::Value;
use std::result::Result;

/// Core tool interface for MCP tools
pub trait Tool {
    /// Get the tool name
    fn name(&self) -> &str;
    
    /// Get the tool description
    fn description(&self) -> &str;
    
    /// Get the input schema for the tool
    fn input_schema(&self) -> Value;
    
    /// Execute the tool with given input
    fn execute(&self, input: Value) -> Result<Value, ToolError>;
    
    /// Validate input against the tool's schema
    fn validate_input(&self, input: &Value) -> Result<(), ValidationError>;
}

/// Tool execution error
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationError),
    
    #[error("Execution error: {message}")]
    ExecutionError { message: String },
    
    #[error("Timeout error: tool execution timed out")]
    TimeoutError,
    
    #[error("Permission error: {message}")]
    PermissionError { message: String },
    
    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error),
}

/// Input validation error
#[derive(Debug, Clone, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
    
    #[error("Missing required field: {field}")]
    MissingField { field: String },
    
    #[error("Invalid field type: {field} expected {expected}")]
    InvalidType { field: String, expected: String },
    
    #[error("Value out of range: {field}")]
    OutOfRange { field: String },
}
