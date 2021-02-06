use crate::data_access::sqlite::patch::patch::Patch;
use rusqlite::{Connection, NO_PARAMS};
use std::error::Error;

/// Patch for adding a work log work_item status column to the logs table.
pub struct Patch2;

impl Patch for Patch2 {
    fn version(&self) -> i32 {
        2
    }

    fn patch(&self, con: &mut Connection) -> Result<(), Box<dyn Error>> {
        let transaction = con.transaction()?;

        transaction.execute(
            "ALTER TABLE logs ADD COLUMN status VARCHAR DEFAULT 'DONE'",
            NO_PARAMS,
        )?;

        transaction.commit()?;

        Ok(())
    }
}
