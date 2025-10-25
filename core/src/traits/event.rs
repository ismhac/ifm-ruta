//! Event system interface

use serde_json::Value;
use std::result::Result;
use std::time::SystemTime;

/// Event bus interface
pub trait EventBus {
    /// Subscribe to events of a specific type
    fn subscribe(&mut self, event_type: EventType, listener: Box<dyn EventListener>);

    /// Unsubscribe from events
    fn unsubscribe(&mut self, event_type: EventType, listener_id: &str);

    /// Publish an event
    fn publish(&self, event: Event) -> Result<(), EventError>;
}

/// Event listener interface
pub trait EventListener {
    /// Handle an event
    fn handle_event(&self, event: &Event) -> Result<(), EventError>;

    /// Get the listener ID
    fn listener_id(&self) -> &str;
}

/// Event structure
#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub data: Value,
    pub timestamp: SystemTime,
    pub source: String,
}

/// Event types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    ToolExecuted,
    ProcessSpawned,
    ProcessCompleted,
    SettingsChanged,
    ErrorOccurred,
    UIEvent,
}

/// Event system error
#[derive(Debug, thiserror::Error)]
pub enum EventError {
    #[error("Event listener not found: {id}")]
    ListenerNotFound { id: String },

    #[error("Event handling failed: {message}")]
    HandlingFailed { message: String },

    #[error("Event publishing failed: {message}")]
    PublishingFailed { message: String },

    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error),
}
