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

    /// Get local date time for the events created timestamp.
    pub fn get_local_date_time(&self) -> chrono::DateTime<chrono::Local> {
        let date_time: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_utc(
            chrono::NaiveDateTime::from_timestamp(self.timestamp() / 1000, 0),
            chrono::Utc,
        );

        // Adjust UTC date time to the local timezone
        chrono::DateTime::from(date_time)
    }
}
