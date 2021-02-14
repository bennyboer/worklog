use std::error::Error;

use work_item::WorkItem;

use crate::work_item::Status;

mod data_access;
pub mod work_item;

/// Log a work work_item.
/// Will return the ID of the new item.
pub fn log_item(item: WorkItem) -> Result<i32, Box<dyn Error>> {
    let mut data_access = data_access::get_data_access()?;

    Ok(data_access.log_item(item)?)
}

/// Update a bunch of work items.
pub fn update_items(items: Vec<&WorkItem>) -> Result<(), Box<dyn Error>> {
    let mut data_access = data_access::get_data_access()?;

    Ok(data_access.update_items(items)?)
}

pub fn list_items() -> Result<Vec<WorkItem>, Box<dyn Error>> {
    let data_access = data_access::get_data_access()?;

    Ok(data_access.list_items()?)
}

pub fn find_items_by_timerange(
    from_timestamp: i64,
    to_timestamp: i64,
) -> Result<Vec<WorkItem>, Box<dyn Error>> {
    let data_access = data_access::get_data_access()?;

    Ok(data_access.filter_items(from_timestamp, to_timestamp)?)
}

pub fn find_item_by_id(id: i32) -> Result<Option<WorkItem>, Box<dyn Error>> {
    let data_access = data_access::get_data_access()?;

    Ok(data_access.find_item_by_id(id)?)
}

pub fn find_items_by_status(status: Status) -> Result<Vec<WorkItem>, Box<dyn Error>> {
    let data_access = data_access::get_data_access()?;

    Ok(data_access.find_items_by_status(status)?)
}

pub fn delete_item(id: i32) -> Result<Option<WorkItem>, Box<dyn Error>> {
    let mut data_access = data_access::get_data_access()?;

    Ok(data_access.delete_item(id)?)
}

pub fn clear() -> Result<(), Box<dyn Error>> {
    let mut data_access = data_access::get_data_access()?;

    Ok(data_access.clear()?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
