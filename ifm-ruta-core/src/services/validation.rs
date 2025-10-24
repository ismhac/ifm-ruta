//! Input validation utilities

use std::collections::HashSet;
use crate::traits::{ValidationError, ValidationRule};

/// Input validator
pub struct InputValidator {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl InputValidator {
    /// Create a new input validator
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }
    
    /// Add a validation rule
    pub fn add_rule(&mut self, rule: Box<dyn ValidationRule>) {
        self.rules.push(rule);
    }
    
    /// Validate input against all rules
    pub fn validate(&self, input: &str) -> Result<(), ValidationError> {
        for rule in &self.rules {
            rule.validate(input)?;
        }
        Ok(())
    }
}

/// Command validation rule
pub struct CommandValidationRule {
    allowed_commands: HashSet<String>,
}

impl CommandValidationRule {
    /// Create a new command validation rule
    pub fn new(allowed_commands: HashSet<String>) -> Self {
        Self { allowed_commands }
    }
}

impl ValidationRule for CommandValidationRule {
    fn validate(&self, input: &str) -> Result<(), ValidationError> {
        // Check for path traversal
        if input.contains("..") {
            return Err(ValidationError::InvalidInput {
                message: "Path traversal detected".to_string(),
            });
        }
        
        // Check for dangerous characters
        if input.contains(";") || input.contains("&") || input.contains("|") {
            return Err(ValidationError::InvalidInput {
                message: "Dangerous characters detected".to_string(),
            });
        }
        
        // Check if command is allowed
        let command = input.split_whitespace().next().unwrap_or("");
        if !self.allowed_commands.is_empty() && !self.allowed_commands.contains(command) {
            return Err(ValidationError::InvalidInput {
                message: format!("Command not allowed: {}", command),
            });
        }
        
        Ok(())
    }
    
    fn rule_name(&self) -> &str {
        "CommandValidationRule"
    }
}

/// Path validation rule
pub struct PathValidationRule;

impl PathValidationRule {
    /// Create a new path validation rule
    pub fn new() -> Self {
        Self
    }
}

impl ValidationRule for PathValidationRule {
    fn validate(&self, input: &str) -> Result<(), ValidationError> {
        // Check for path traversal
        if input.contains("..") {
            return Err(ValidationError::InvalidInput {
                message: "Path traversal detected".to_string(),
            });
        }
        
        // Check for absolute paths (optional restriction)
        if input.starts_with('/') || input.starts_with('\\') {
            return Err(ValidationError::InvalidInput {
                message: "Absolute paths not allowed".to_string(),
            });
        }
        
        Ok(())
    }
    
    fn rule_name(&self) -> &str {
        "PathValidationRule"
    }
}
