use super::PATCHES;
use rusqlite::{params, Connection};
use std::error::Error;

/// Patcher patching a database.
pub struct Patcher<'a> {
    /// Connection to the database to patch.
    connection: &'a mut Connection,
}

impl Patcher<'_> {
    /// Create a new patcher to work on the given connection.
    pub fn new(connection: &mut Connection) -> Patcher {
        Patcher { connection }
    }

    /// Patch the database.
    pub fn patch(&mut self, from_version: i32, to_version: i32) -> Result<(), Box<dyn Error>> {
        for patch in &PATCHES {
            if to_version <= from_version || from_version >= patch.version() {
                break; // No more patches needed
            }

            patch.patch(self.connection)?;
            self.change_version(patch.version())?;
        }

        Ok(())
    }

    /// Change the version of the database.
    fn change_version(&self, version: i32) -> Result<(), Box<dyn Error>> {
        self.connection
            .execute("UPDATE info SET version = ?1", params![version])?;

        Ok(())
    }
}
