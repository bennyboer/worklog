use crate::work_item::event::EventType;

/// Event that may occur on a work item.
#[derive(Debug)]
pub struct Event {
    /// Type of the event.
    event_type: EventType,
    /// Timestamp this event occurred.
    timestamp: i64,
}

impl Event {
    /// Create a new event.
    pub fn new(event_type: EventType, timestamp: i64) -> Event {
        Event {
            event_type,
            timestamp,
        }
    }

    /// Get the events type.
    pub fn event_type(&self) -> EventType {
        self.event_type
    }

    /// Get the events timestamp.
    /// When did the event occur?
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
}
