use rusqlite::Connection;
use std::error::Error;

/// Patch that will patch the database to another version.
pub trait Patch {
    /// Get the version the patch will patch the database to.
    fn version(&self) -> i32;

    /// Try to patch the database to the patches version.
    fn patch(&self, con: &mut Connection) -> Result<(), Box<dyn Error>>;
}
