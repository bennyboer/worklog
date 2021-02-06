use crate::work_item::WorkItem;
use std::error::Error;

/// Common data access interface.
pub trait DataAccess {
    /// Log a work work_item.
    fn log_item(&mut self, entry: WorkItem) -> Result<(), Box<dyn Error>>;

    /// List all available work items.
    fn list_items(&self) -> Result<Vec<WorkItem>, Box<dyn Error>>;

    /// List available work items in the given time range.
    fn filter_items(
        &self,
        from_timestamp: i64,
        to_timestamp: i64,
    ) -> Result<Vec<WorkItem>, Box<dyn Error>>;

    /// Find a work item by its ID.
    fn find_item_by_id(&self, id: i32) -> Result<Option<WorkItem>, Box<dyn Error>>;
}
