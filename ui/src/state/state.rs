use crate::state::work_item::UiWorkItem;
use druid::{Data, Lens};
use std::sync::Arc;

/// Global state of the UI.
#[derive(Clone, Data, Lens)]
pub(crate) struct UiState {
    pub test: String,
    /// Reference to a list of work items.
    pub work_items: Arc<Vec<UiWorkItem>>,
}
