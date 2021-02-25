use druid::{Data, Lens};

/// Work item displayable in the UI.
#[derive(Clone, Data, Lens)]
pub(crate) struct UiWorkItem {
    /// ID of the work item.
    pub id: i32,
    /// Description of the work item.
    pub description: String,
    // TODO Add more
}
