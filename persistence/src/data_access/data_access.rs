use crate::entry::LogEntry;
use std::error::Error;

/// Common data access interface.
pub trait DataAccess {
    /// Log a work entry.
    fn log_entry(&mut self, entry: LogEntry) -> Result<(), Box<dyn Error>>;

    /// List all available entries.
    fn list_entries(&self) -> Result<Vec<LogEntry>, Box<dyn Error>>;

    /// List available entries in the given time range.
    fn filter_entries(
        &self,
        from_timestamp: i64,
        to_timestamp: i64,
    ) -> Result<Vec<LogEntry>, Box<dyn Error>>;
}
