use std::error::Error;
use std::{fs, path};

use rusqlite::{params, Connection, Rows, NO_PARAMS};

use crate::data_access::sqlite::patch::Patcher;
use crate::data_access::DataAccess;
use crate::work_item::{Status, WorkItem};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

/// Latest database version to patch to.
const LATEST_VERSION: i32 = 2;

/// Directory under the HOME directory of the current user where
/// to store the logs database.
const SUB_HOME_DIRECTORY: &str = ".worklog";

/// File name of the logs database.
const FILE_NAME: &str = "logs.db";

/// Data access using SQLite.
pub struct SQLiteDataAccess {
    connection: Connection,
}

/// Determine the path to the logs database.
fn determine_database_path() -> Result<path::PathBuf, &'static str> {
    Ok(match home::home_dir() {
        Some(path) => Ok(path),
        None => Err("Could not determine the current users HOME directory"),
    }?
    .join(SUB_HOME_DIRECTORY)
    .join(FILE_NAME))
}

impl SQLiteDataAccess {
    /// Create a new SQLite data access.
    pub fn new() -> Result<SQLiteDataAccess, Box<dyn Error>> {
        let db_path = determine_database_path()?;

        // Create directories if they do not exist
        fs::create_dir_all(&db_path.parent().unwrap())?;

        let mut data_access = SQLiteDataAccess {
            connection: Connection::open(&db_path)?,
        };

        data_access.prepare_database()?;

        Ok(data_access)
    }

    /// Prepare the database for usage.
    fn prepare_database(&mut self) -> Result<(), Box<dyn Error>> {
        // Check info table for the database version
        let version = match self.check_version() {
            Ok(version) => version,
            Err(_) => -1, // Database not initialized yet
        };

        let init_db = version < 0;
        if init_db {
            self.initialize_database()?;
        }

        let version = self.check_version()?;
        if version < LATEST_VERSION {
            let mut patcher = Patcher::new(&mut self.connection);
            patcher.patch(version, LATEST_VERSION)?;
        }

        Ok(())
    }

    /// Check the current database version.
    fn check_version(&self) -> Result<i32, Box<dyn Error>> {
        let version: i32 =
            self.connection
                .query_row("SELECT version FROM info", NO_PARAMS, |row| row.get(0))?;

        Ok(version)
    }

    /// Initialize the database with a info table.
    fn initialize_database(&self) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "CREATE TABLE info (
                version INTEGER PRIMARY KEY
             )",
            params![],
        )?;

        self.connection
            .execute("INSERT INTO info (version) VALUES (?1)", params![0])?;

        Ok(())
    }
}

impl DataAccess for SQLiteDataAccess {
    fn log_item(&mut self, entry: WorkItem) -> Result<i32, Box<dyn Error>> {
        let transaction = self.connection.transaction()?;

        // Insert log work_item information
        transaction.execute(
            "INSERT INTO logs (description, time_taken, timestamp, status) VALUES (?1, ?2, ?3, ?4)",
            params![
                entry.description(),
                entry.time_taken(),
                entry.timestamp(),
                format!("{}", entry.status())
            ],
        )?;

        // Check ID of the new log work_item
        let id: i32 =
            transaction.query_row("SELECT last_insert_rowid()", NO_PARAMS, |row| row.get(0))?;

        // Insert tags in the log_tags table
        for tag in entry.tags() {
            transaction.execute(
                "INSERT INTO log_tags (log_id, tag) VALUES (?1, ?2)",
                params![id, tag],
            )?;
        }

        transaction.commit()?;

        Ok(id)
    }

    fn list_items(&self) -> Result<Vec<WorkItem>, Box<dyn Error>> {
        let mut statement = self.connection.prepare(
            "SELECT logs.id, logs.description, logs.time_taken, logs.timestamp, logs.status, log_tags.tag \
            FROM logs, log_tags \
            WHERE logs.id = log_tags.log_id",
        )?;

        return rows_to_items(statement.query(NO_PARAMS)?);
    }

    fn filter_items(
        &self,
        from_timestamp: i64,
        to_timestamp: i64,
    ) -> Result<Vec<WorkItem>, Box<dyn Error>> {
        let mut statement = self.connection.prepare(
            "SELECT logs.id, logs.description, logs.time_taken, logs.timestamp, logs.status, log_tags.tag \
            FROM logs, log_tags \
            WHERE logs.id = log_tags.log_id AND logs.timestamp >= ?1 AND logs.timestamp < ?2",
        )?;

        return rows_to_items(statement.query(params![from_timestamp, to_timestamp])?);
    }

    fn find_item_by_id(&self, id: i32) -> Result<Option<WorkItem>, Box<dyn Error>> {
        let mut statement = self.connection.prepare(
            "SELECT logs.id, logs.description, logs.time_taken, logs.timestamp, logs.status, log_tags.tag \
            FROM logs, log_tags \
            WHERE logs.id = log_tags.log_id AND logs.id = ?1",
        )?;

        let mut items = rows_to_items(statement.query(params![id])?)?;

        Ok(items.pop())
    }
}

/// Fetch all log entries from the passed rows.
fn rows_to_items(mut rows: Rows) -> Result<Vec<WorkItem>, Box<dyn Error>> {
    let mut entry_map = HashMap::<i32, WorkItem>::new();

    while let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        let tag = row.get(5)?;

        if entry_map.contains_key(&id) {
            let entry = entry_map.get_mut(&id).unwrap();
            entry.push_tag(tag);
        } else {
            let description = row.get(1)?;
            let time_taken = row.get(2)?;
            let timestamp = row.get(3)?;
            let status: String = row.get(4)?;

            let mut tag_set = HashSet::new();
            tag_set.insert(tag);

            let entry = WorkItem::new_internal(
                id,
                description,
                tag_set,
                time_taken,
                Status::from_str(&status)
                    .expect("Status string in database table logs could not be interpreted"),
                timestamp,
            );

            entry_map.insert(id, entry);
        }
    }

    Ok(entry_map.into_iter().map(|(_key, value)| value).collect())
}
