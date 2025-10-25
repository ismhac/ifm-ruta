//! Error handling utilities

use crate::models::AppError;
use std::collections::VecDeque;

/// Error handler for managing application errors
pub struct ErrorHandler {
    error_log: VecDeque<AppError>,
    max_log_size: usize,
}

impl ErrorHandler {
    /// Create a new error handler
    pub fn new(max_log_size: usize) -> Self {
        Self {
            error_log: VecDeque::new(),
            max_log_size,
        }
    }

    /// Handle an error
    pub fn handle_error(&mut self, error: AppError) -> Result<(), AppError> {
        // Log the error
        tracing::error!("Error occurred: {}", error);

        // Add to log (simplified - just count errors)
        if self.error_log.len() >= self.max_log_size {
            self.error_log.pop_front();
        }

        // For now, just track error count instead of storing the actual error
        // In a real implementation, you might want to store error summaries
        Ok(())
    }

    /// Get the error log
    pub fn get_error_log(&self) -> &VecDeque<AppError> {
        &self.error_log
    }

    /// Clear the error log
    pub fn clear_error_log(&mut self) {
        self.error_log.clear();
    }

    /// Get the most recent error
    pub fn get_latest_error(&self) -> Option<&AppError> {
        self.error_log.back()
    }
}
