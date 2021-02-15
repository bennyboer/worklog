use crate::data_access::sqlite::patch::patch::Patch;
use rusqlite::{Connection, NO_PARAMS};
use std::error::Error;

/// Patch for adding the basic tables logs, log_tags, log_events.
pub struct Patch1;

impl Patch for Patch1 {
    fn version(&self) -> i32 {
        1
    }

    fn patch(&self, con: &mut Connection) -> Result<(), Box<dyn Error>> {
        let transaction = con.transaction()?;

        // Add logs table
        transaction.execute(
            "CREATE TABLE logs (\
            id INTEGER PRIMARY KEY, \
            description TEXT NOT NULL, \
            status TEXT NOT NULL\
            )",
            NO_PARAMS,
        )?;

        // Add log_tags table
        transaction.execute(
            "CREATE TABLE log_tags (\
            log_id INTEGER NOT NULL, \
            tag TEXT NOT NULL, \
            PRIMARY KEY (log_id, tag), \
            FOREIGN KEY (log_id) REFERENCES logs(id)\
            )",
            NO_PARAMS,
        )?;

        // Add log_events table
        transaction.execute(
            "CREATE TABLE log_events (\
            log_id INTEGER NOT NULL, \
            timestamp INTEGER NOT NULL, \
            event TEXT NOT NULL, \
            PRIMARY KEY (log_id, timestamp, event), \
            FOREIGN KEY (log_id) REFERENCES logs(id)\
            )",
            NO_PARAMS,
        )?;

        transaction.commit()?;

        Ok(())
    }
}
