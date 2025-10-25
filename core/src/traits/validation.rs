//! Validation traits and types

use crate::traits::ValidationError;

/// Validation rule trait
pub trait ValidationRule {
    /// Validate input against the rule
    fn validate(&self, input: &str) -> Result<(), ValidationError>;

    /// Get the rule name
    fn rule_name(&self) -> &str;
}
