use druid::{Data, Lens};
use std::rc::Rc;

/// Work item displayable in the UI.
#[derive(Clone, Data, Lens)]
pub struct UiWorkItem {
    /// ID of the work item.
    pub id: i32,
    /// Description of the work item.
    pub description: String,
    /// Status of the work item.
    pub status: UiWorkItemStatus,
    /// Tags of the item.
    pub tags: im::Vector<String>,
    /// Reference to the original work item.
    pub work_item: Rc<persistence::calc::WorkItem>,
}

#[derive(Clone, Data, PartialEq)]
pub enum UiWorkItemStatus {
    InProgress,
    Paused,
    Finished,
}
