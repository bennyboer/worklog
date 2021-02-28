use std::collections::HashSet;

use crate::calc::event::{Event, EventType};
use crate::calc::Status;

#[derive(Debug)]
pub struct WorkItem {
    /// ID of the work item (only present when already stored in the database).
    id: Option<i32>,
    /// Description of the work item.
    description: String,
    /// Tags to further classify the work item.
    tags: HashSet<String>,
    /// Status of the work item.
    status: Status,
    /// Events the work item was undergoing in its lifetime.
    /// They must be sorted by their timestamp all the time.
    events: Vec<Event>,
}

impl WorkItem {
    /// Create a new log calc.
    pub fn new(description: String, status: Status, tags: HashSet<String>) -> WorkItem {
        WorkItem {
            id: None,
            description,
            status,
            tags,
            events: vec![Event::new(EventType::Started, get_current_timestamp())],
        }
    }

    /// Create a new log calc for internal use.
    pub fn new_internal(
        id: i32,
        description: String,
        status: Status,
        tags: HashSet<String>,
        events: Vec<Event>,
    ) -> WorkItem {
        WorkItem {
            id: Some(id),
            description,
            status,
            tags,
            events,
        }
    }

    /// Get the work items ID.
    pub fn id(&self) -> Option<i32> {
        return self.id;
    }

    /// Get the work items description.
    pub fn description(&self) -> &String {
        &self.description
    }

    /// Set the work items description.
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Get a list of alphabetically sorted tags for the work item.
    pub fn tags(&self) -> Vec<String> {
        let mut arr: Vec<String> = self.tags.iter().map(|s| s.to_owned()).collect();
        arr.sort();

        return arr;
    }

    /// Set the tags describing the work item.
    pub fn set_tags(&mut self, tags: HashSet<String>) {
        self.tags = tags;
    }

    /// Push a tag describing the work item.
    /// This will be added to the already existing tags and not override them.
    pub fn push_tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }

    /// Get the time the user has been working on the work item (in milliseconds).
    pub fn time_taken(&self) -> i64 {
        let mut time_taken: i64 = 0;

        let mut cur_start_timestamp: Option<i64> = None;
        for event in &self.events {
            match event.event_type() {
                EventType::Started | EventType::Continued => {
                    cur_start_timestamp = Some(event.timestamp())
                }
                EventType::Paused | EventType::Finished => {
                    // When work item has been finished after paused,
                    // there is no cur_start_timestamp.
                    if let Some(start) = cur_start_timestamp {
                        let end = event.timestamp();

                        time_taken += end - start;
                        cur_start_timestamp = None;
                    }
                }
            }
        }

        // Check whether item is currently in progress and calculate time taken to now!
        if let Status::InProgress = self.status {
            time_taken +=
                chrono::Utc::now().timestamp_millis() - self.events.last().unwrap().timestamp();
        }

        return time_taken;
    }

    /// Get the timestamp the item was created.
    pub fn created_timestamp(&self) -> i64 {
        // Get first Created event
        for event in &self.events {
            if let EventType::Started = event.event_type() {
                return event.timestamp();
            }
        }

        panic!("A work item must have a created event!");
    }

    /// Get the current status of the work item.
    pub fn status(&self) -> Status {
        return self.status;
    }

    /// Get all events the work item was undergoing to this moment sorted by the
    /// events timestamp.
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    /// Set the events of the item.
    pub fn set_events(&mut self, events: Vec<Event>) {
        self.events = events;
    }

    /// Pause working on the work item.
    /// Will result in an error if the work item is in an invalid state.
    pub fn pause_working(&mut self) -> Result<(), &'static str> {
        if let Status::InProgress = self.status {
            self.status = Status::Paused;

            // Add paused event
            self.events
                .push(Event::new(EventType::Paused, get_current_timestamp()));

            Ok(())
        } else {
            Err("Can only pause work items that are currently in progress!")
        }
    }

    /// Continue working on the work item.
    /// Will result in an error if the work item is in an invalid state.
    pub fn continue_working(&mut self) -> Result<(), &'static str> {
        if let Status::Paused = self.status {
            self.status = Status::InProgress;

            // Add continue event
            self.events
                .push(Event::new(EventType::Continued, get_current_timestamp()));

            Ok(())
        } else {
            Err("Can only continue working on work items that are currently paused!")
        }
    }

    /// Finish working on the work item.
    /// Will result in an error if the work item is in an invalid state.
    pub fn finish_working(&mut self, timestamp: Option<i64>) -> Result<(), &'static str> {
        match self.status {
            Status::InProgress | Status::Paused => {
                self.status = Status::Done;

                // Add finished event
                self.events.push(Event::new(
                    EventType::Finished,
                    timestamp.unwrap_or(get_current_timestamp()),
                ));

                Ok(())
            }
            Status::Done => Err("Cannot finish working on work item already finished!"),
        }
    }
}

/// Get the current UTC+0 timestamp in milliseconds.
fn get_current_timestamp() -> i64 {
    chrono::Utc::now().timestamp_millis()
}
