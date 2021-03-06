use crate::state::work_item;
use crate::state::work_item::UiWorkItem;
use druid::im;
use druid::{Data, Lens};
use persistence::calc::Status;
use std::cell::RefCell;
use std::error::Error;
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
    /// A currently selected work item.
    pub selected_work_item: Option<Rc<RefCell<UiWorkItem>>>,
    /// Reference to a list of work items.
    /// If None, the work items have not yet been loaded for the set date.
    pub work_items: Option<DayViewWorkItems>,
}

pub(crate) struct SelectedWorkItemLens;

impl Lens<Rc<RefCell<UiWorkItem>>, UiWorkItem> for SelectedWorkItemLens {
    fn with<R, F: FnOnce(&UiWorkItem) -> R>(&self, data: &Rc<RefCell<UiWorkItem>>, f: F) -> R {
        f(&data.as_ref().borrow())
    }

    fn with_mut<R, F: FnOnce(&mut UiWorkItem) -> R>(
        &self,
        data: &mut Rc<RefCell<UiWorkItem>>,
        f: F,
    ) -> R {
        f(&mut data.as_ref().borrow_mut())
    }
}

impl DayViewState {
    /// Create new day view state for the given initial date.
    pub fn new(date: chrono::Date<chrono::Local>) -> DayViewState {
        DayViewState {
            date: Rc::new(date),
            selected_work_item: None,
            work_items: load_work_items(&date).expect("Could not load work items"),
        }
    }

    /// Update the day view state for the given date.
    pub fn update(&mut self, new_date: chrono::Date<chrono::Local>) {
        self.unselect();
        self.date = Rc::from(new_date);
        self.work_items = load_work_items(&new_date).expect("Could not load work items")
    }

    /// Select a work item with the given ID.
    pub fn select_item(&mut self, id: i32) {
        if let Some(item_ref) = self.find_item_with_id(id) {
            self.selected_work_item = Some(item_ref);
        }
    }

    /// Find a work item with the given ID.
    fn find_item_with_id(&self, id: i32) -> Option<Rc<RefCell<UiWorkItem>>> {
        if let Some(items) = &self.work_items {
            for item_ref in &items.items {
                let item = item_ref.borrow();
                if item.id == id {
                    return Some(Rc::clone(item_ref));
                }
            }
        }

        return None;
    }

    /// Unselect a previously selected work item.
    pub fn unselect(&mut self) {
        self.selected_work_item = None;
    }
}

/// Items of a day view.
#[derive(Clone, Data, Lens)]
pub(crate) struct DayViewWorkItems {
    /// Work items in the day view.
    pub items: im::Vector<Rc<RefCell<UiWorkItem>>>,
}

/// Load work items for the given date.
fn load_work_items(
    date: &chrono::Date<chrono::Local>,
) -> Result<Option<DayViewWorkItems>, Box<dyn Error>> {
    let from_timestamp = date.and_hms(0, 0, 0).timestamp_millis();
    let to_timestamp = date.succ().and_hms(0, 0, 0).timestamp_millis();

    let items = persistence::find_items_by_timerange(from_timestamp, to_timestamp)?;

    if items.is_empty() {
        return Ok(None);
    }

    let mut ui_work_items = im::Vector::new();
    for item in items {
        ui_work_items.push_back(Rc::new(RefCell::new(work_item::UiWorkItem {
            id: item.id().unwrap(),
            description: item.description().to_owned(),
            status: match item.status() {
                Status::Done => work_item::UiWorkItemStatus::Finished,
                Status::InProgress => work_item::UiWorkItemStatus::InProgress,
                Status::Paused => work_item::UiWorkItemStatus::Paused,
            },
            tags: im::Vector::from(item.tags()),
            work_item: Rc::new(RefCell::new(item)),
            tmp: String::new(),
        })));
    }

    Ok(Some(DayViewWorkItems {
        items: ui_work_items,
    }))
}
