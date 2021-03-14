use druid::{Data, Lens};
use std::cell::RefCell;
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
    pub work_item: Rc<RefCell<persistence::calc::WorkItem>>,
    /// Temporary string used for example to add a new tag to the tag list.
    pub tmp: String,
}

#[derive(Clone, Data, PartialEq, Debug)]
pub enum UiWorkItemStatus {
    InProgress,
    Paused,
    Finished,
}
