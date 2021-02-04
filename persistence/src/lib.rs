use entry::LogEntry;
use std::error::Error;

mod data_access;
pub mod entry;

/// Log a work entry.
pub fn log_entry(entry: LogEntry) -> Result<(), Box<dyn Error>> {
    let mut data_access = data_access::get_data_access()?;

    data_access.log_entry(entry)?;

    Ok(())
}

pub fn list_entries() -> Result<Vec<LogEntry>, Box<dyn Error>> {
    let data_access = data_access::get_data_access()?;

    Ok(data_access.list_entries()?)
}

pub fn filter_entries(
    from_timestamp: i64,
    to_timestamp: i64,
) -> Result<Vec<LogEntry>, Box<dyn Error>> {
    let data_access = data_access::get_data_access()?;

    Ok(data_access.filter_entries(from_timestamp, to_timestamp)?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
