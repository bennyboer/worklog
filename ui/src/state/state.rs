use crate::state::work_item::UiWorkItem;
use druid::im;
use druid::{Data, Lens};
use std::rc::Rc;

/// Global state of the UI.
#[derive(Clone, Data, Lens)]
pub(crate) struct UiState {
    /// State of the currently visible day view.
    pub day: DayViewState,
}

/// State for a date.
#[derive(Clone, Data, Lens)]
pub(crate) struct DayViewState {
    /// Date currently displayed.
    pub date: Rc<chrono::Date<chrono::Local>>,
    /// Reference to a list of work items.
    /// If None, the work items have not yet been loaded for the set date.
    pub work_items: Option<DayViewWorkItems>,
}

/// Items of a day view.
#[derive(Clone, Data, Lens)]
pub(crate) struct DayViewWorkItems {
    /// Work items in the day view.
    pub items: im::Vector<UiWorkItem>,
}
