use std::error::Error;

use crate::work_item::{Status, WorkItem};

/// Common data access interface.
pub trait DataAccess {
    /// Log a work work_item.
    /// Will return the ID of the new work item.
    fn log_item(&mut self, item: WorkItem) -> Result<i32, Box<dyn Error>>;

    /// Update a bunch of work items.
    fn update_items(&mut self, items: Vec<&WorkItem>) -> Result<(), Box<dyn Error>>;

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

    /// Find work items by the given status.
    fn find_items_by_status(&self, status: Status) -> Result<Vec<WorkItem>, Box<dyn Error>>;

    /// Delete a work item with the given ID.
    /// Returns the deleted work item or None if there is no item with the given ID.
    fn delete_item(&mut self, id: i32) -> Result<Option<WorkItem>, Box<dyn Error>>;
}
