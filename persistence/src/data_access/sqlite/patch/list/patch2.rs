use crate::data_access::sqlite::patch::patch::Patch;
use rusqlite::{Connection, NO_PARAMS};
use std::error::Error;

/// Patch for adding a work log work item status and temporary timestamp column to the logs table.
/// This will enable starting, stopping and pausing work items.
pub struct Patch2;

impl Patch for Patch2 {
    fn version(&self) -> i32 {
        2
    }

    fn patch(&self, con: &mut Connection) -> Result<(), Box<dyn Error>> {
        let transaction = con.transaction()?;

        // Add status column
        transaction.execute(
            "ALTER TABLE logs ADD COLUMN status VARCHAR NOT NULL DEFAULT 'DONE'",
            NO_PARAMS,
        )?;

        // Add timer timestamp column.
        // This column will hold a timestamp of when the "timer" has been started
        // or paused to get the work item duration from when calling finish.
        transaction.execute(
            "ALTER TABLE logs ADD COLUMN timer_timestamp INTEGER NOT NULL DEFAULT -1",
            NO_PARAMS,
        )?;

        transaction.commit()?;

        Ok(())
    }
}
