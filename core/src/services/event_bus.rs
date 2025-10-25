//! Event bus implementation

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::traits::{Event, EventBus, EventError, EventListener, EventType};

/// Event bus implementation
pub struct EventBusImpl {
    listeners: Arc<Mutex<HashMap<EventType, Vec<Box<dyn EventListener>>>>>,
    event_queue: Arc<Mutex<Vec<Event>>>,
}

impl EventBusImpl {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(HashMap::new())),
            event_queue: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl EventBus for EventBusImpl {
    fn subscribe(&mut self, event_type: EventType, listener: Box<dyn EventListener>) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners
            .entry(event_type)
            .or_insert_with(Vec::new)
            .push(listener);
    }

    fn unsubscribe(&mut self, event_type: EventType, listener_id: &str) {
        let mut listeners = self.listeners.lock().unwrap();
        if let Some(listener_list) = listeners.get_mut(&event_type) {
            listener_list.retain(|listener| listener.listener_id() != listener_id);
        }
    }

    fn publish(&self, event: Event) -> Result<(), EventError> {
        let listeners = self.listeners.lock().unwrap();

        if let Some(listener_list) = listeners.get(&event.event_type) {
            for listener in listener_list {
                if let Err(e) = listener.handle_event(&event) {
                    return Err(EventError::HandlingFailed {
                        message: e.to_string(),
                    });
                }
            }
        }

        // Add to event queue for debugging/logging
        let mut queue = self.event_queue.lock().unwrap();
        queue.push(event);

        Ok(())
    }
}
