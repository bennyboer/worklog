use chrono::Utc;
use std::collections::HashSet;

#[derive(Debug)]
pub struct LogEntry {
    /// Description of the work log entry.
    description: String,
    /// Tags to further classify the log entry.
    tags: HashSet<String>,
    /// Time spent on the task (in seconds).
    time_taken: i32,
    /// Timestamp of when the task was entered into the system.
    timestamp: i64,
}

impl LogEntry {
    /// Create a new log entry.
    pub fn new(description: String, tags: HashSet<String>, time_taken: i32) -> LogEntry {
        LogEntry {
            description,
            tags,
            time_taken,
            timestamp: Utc::now().timestamp_millis(),
        }
    }

    /// Create a new log entry.
    pub fn new_internal(
        description: String,
        tags: HashSet<String>,
        time_taken: i32,
        timestamp: i64,
    ) -> LogEntry {
        LogEntry {
            description,
            tags,
            time_taken,
            timestamp,
        }
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

    pub fn push_tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }
}
