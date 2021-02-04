mod data_access;
mod data_access_factory;
mod sqlite;

pub use data_access::DataAccess;
pub use data_access_factory::get_data_access;
