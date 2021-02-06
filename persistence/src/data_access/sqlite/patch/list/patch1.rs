use crate::data_access::sqlite::patch::patch::Patch;
use rusqlite::{Connection, NO_PARAMS};
use std::error::Error;

/// Patch for adding the logs table.
pub struct Patch1;

impl Patch for Patch1 {
    fn version(&self) -> i32 {
        1
    }

    fn patch(&self, con: &mut Connection) -> Result<(), Box<dyn Error>> {
        let transaction = con.transaction()?;

        transaction.execute(
            "CREATE TABLE logs (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                time_taken INTEGER NOT NULL,
                timestamp INTEGER NOT NULL
             )",
            NO_PARAMS,
        )?;

        transaction.execute(
            "CREATE TABLE log_tags (
                log_id INTEGER NOT NULL,
                tag TEXT NOT NULL,
                PRIMARY KEY (log_id, tag),
                FOREIGN KEY (log_id) REFERENCES logs(id)
            )",
            NO_PARAMS,
        )?;

        transaction.commit()?;

        Ok(())
    }
}
