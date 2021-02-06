use crate::work_item::Status;
use chrono::Utc;
use std::collections::HashSet;

#[derive(Debug)]
pub struct WorkItem {
    /// ID of the work item (only present when already stored in the database).
    id: Option<i32>,
    /// Description of the work item.
    description: String,
    /// Tags to further classify the work item.
    tags: HashSet<String>,
    /// Time spent on the work item (in seconds).
    time_taken: i32,
    /// Status of the work item.
    status: Status,
    /// Timestamp of when the work item was entered into the system.
    timestamp: i64,
    /// Timestamp of the work items timer (if any).
    /// Used to measure the duration it took to complete a work item.
    timer_timestamp: Option<i64>,
}

impl WorkItem {
    /// Create a new log work_item.
    pub fn new(
        description: String,
        tags: HashSet<String>,
        time_taken: i32,
        status: Status,
    ) -> WorkItem {
        WorkItem {
            id: None,
            description,
            tags,
            time_taken,
            status,
            timestamp: Utc::now().timestamp_millis(),
            timer_timestamp: None,
        }
    }

    /// Create a new log work_item for internal use.
    pub fn new_internal(
        id: i32,
        description: String,
        tags: HashSet<String>,
        time_taken: i32,
        status: Status,
        timestamp: i64,
        timer_timestamp: Option<i64>,
    ) -> WorkItem {
        WorkItem {
            id: Some(id),
            description,
            tags,
            time_taken,
            status,
            timestamp,
            timer_timestamp,
        }
    }

    pub fn id(&self) -> Option<i32> {
        return self.id;
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn tags(&self) -> Vec<String> {
        let mut arr: Vec<String> = self.tags.iter().map(|s| s.to_owned()).collect();
        arr.sort();

        return arr;
    }

    pub fn time_taken(&self) -> i32 {
        return self.time_taken;
    }

    pub fn timestamp(&self) -> i64 {
        return self.timestamp;
    }

    pub fn status(&self) -> &Status {
        return &self.status;
    }

    pub fn timer_timestamp(&self) -> Option<i64> {
        return self.timer_timestamp;
    }

    pub fn push_tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }
}
