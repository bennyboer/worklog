use crate::data_access::sqlite::SQLiteDataAccess;
use crate::data_access::DataAccess;
use std::error::Error;

/// Get the data access to use.
pub fn get_data_access() -> Result<Box<dyn DataAccess>, Box<dyn Error>> {
    Ok(Box::new(SQLiteDataAccess::new()?))
}
