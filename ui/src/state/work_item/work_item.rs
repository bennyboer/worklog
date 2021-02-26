use druid::{Data, Lens};

/// Work item displayable in the UI.
#[derive(Clone, Data, Lens)]
pub struct UiWorkItem {
    /// ID of the work item.
    pub id: i32,
    /// Description of the work item.
    pub description: String,
    /// Status of the work item.
    pub status: UiWorkItemStatus,
    // TODO Add more
}

#[derive(Clone, Data, PartialEq)]
pub enum UiWorkItemStatus {
    InProgress,
    Paused,
    Finished,
}
