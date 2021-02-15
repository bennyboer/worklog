use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str::FromStr;
use std::{fs, path};

use rusqlite::{params, Connection, Rows, Transaction, NO_PARAMS};

use crate::data_access::sqlite::patch::Patcher;
use crate::data_access::DataAccess;
use crate::work_item::event::{Event, EventType};
use crate::work_item::{Status, WorkItem};
use std::collections::hash_map::Entry;

/// Latest database version to patch to.
const LATEST_VERSION: i32 = 1;

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

/// Work item to be filled with more data.
struct TmpWorkItem {
    id: i32,
    description: String,
    status: Status,
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
    fn log_item(&mut self, item: WorkItem) -> Result<i32, Box<dyn Error>> {
        let transaction = self.connection.transaction()?;

        // Insert work item information to logs table
        transaction.execute(
            "INSERT INTO logs (description, status) VALUES (?1, ?2)",
            params![item.description(), format!("{}", item.status())],
        )?;

        // Check ID of the new log work_item
        let id: i32 =
            transaction.query_row("SELECT last_insert_rowid()", NO_PARAMS, |row| row.get(0))?;

        // Insert tags in the log_tags table
        insert_tags(&transaction, id, &item.tags())?;

        // Insert events in the log_events table
        insert_events(&transaction, id, item.events())?;

        transaction.commit()?;

        Ok(id)
    }

    fn update_items(&mut self, items: Vec<&WorkItem>) -> Result<(), Box<dyn Error>> {
        let transaction = self.connection.transaction()?;

        for item in items {
            let id = item.id().expect("ID must be present at this point!");

            // Update in logs table
            transaction.execute(
                "UPDATE logs \
        SET description = ?2, \
        status = ?3 \
        WHERE id = ?1",
                params![id, item.description(), format!("{}", item.status())],
            )?;

            // Delete all tags for the work item in the log_tags table
            delete_tags(&transaction, id)?;

            // Enter new tags in the log_tags table
            insert_tags(&transaction, id, &item.tags())?;

            // Delete all events for the work item in the log_events table
            delete_events(&transaction, id)?;

            // Enter new events in the log_events table
            insert_events(&transaction, id, item.events())?;
        }

        transaction.commit()?;

        Ok(())
    }

    fn list_items(&self) -> Result<Vec<WorkItem>, Box<dyn Error>> {
        // Fetch all tmp work items from the logs table
        let item_lookup = tmp_item_lookup_from_rows(
            self.connection
                .prepare("SELECT id, description, status FROM logs")?
                .query(NO_PARAMS)?,
        )?;

        // Fetch and cache tags for later lookup
        let tags_lookup: HashMap<i32, HashSet<String>> = tags_lookup_from_rows(
            self.connection
                .prepare("SELECT log_id, tag FROM log_tags")?
                .query(NO_PARAMS)?,
        )?;

        // Fetch and cache events for later lookup
        let events_lookup = events_lookup_from_rows(
            self.connection
                .prepare("SELECT log_id, timestamp, event FROM log_events")?
                .query(NO_PARAMS)?,
        )?;

        // Create work items from the cached data
        Ok(create_work_items(item_lookup, tags_lookup, events_lookup))
    }

    fn filter_items(
        &self,
        from_timestamp: i64,
        to_timestamp: i64,
    ) -> Result<Vec<WorkItem>, Box<dyn Error>> {
        // Fetch all tmp work items from the logs table
        let item_lookup = tmp_item_lookup_from_rows(
            self.connection
                .prepare(
                    "SELECT logs.id, logs.description, logs.status \
            FROM logs, log_events \
            WHERE logs.id = log_events.log_id \
                AND log_events.event = 'STARTED' \
                AND log_events.timestamp >= ?1 \
                AND log_events.timestamp < ?2",
                )?
                .query(params![from_timestamp, to_timestamp])?,
        )?;

        // Fetch and cache tags for later lookup
        let tags_lookup: HashMap<i32, HashSet<String>> = tags_lookup_from_rows(
            self.connection
                .prepare(
                    "SELECT log_id, tag \
            FROM log_tags \
            WHERE log_id IN (\
                SELECT logs.id \
                FROM logs, log_events \
                WHERE logs.id = log_events.log_id \
                    AND log_events.event = 'STARTED' \
                    AND log_events.timestamp >= ?1 \
                    AND log_events.timestamp < ?2\
            )",
                )?
                .query(params![from_timestamp, to_timestamp])?,
        )?;

        // Fetch and cache events for later lookup
        let events_lookup = events_lookup_from_rows(
            self.connection
                .prepare(
                    "SELECT log_id, timestamp, event \
            FROM log_events WHERE log_id IN (\
                SELECT logs.id \
                FROM logs, log_events \
                WHERE logs.id = log_events.log_id \
                    AND log_events.event = 'STARTED' \
                    AND log_events.timestamp >= ?1 \
                    AND log_events.timestamp < ?2\
            )",
                )?
                .query(params![from_timestamp, to_timestamp])?,
        )?;

        // Create work items from the cached data
        Ok(create_work_items(item_lookup, tags_lookup, events_lookup))
    }

    fn find_item_by_id(&self, id: i32) -> Result<Option<WorkItem>, Box<dyn Error>> {
        let item_lookup = tmp_item_lookup_from_rows(
            self.connection
                .prepare("SELECT id, description, status FROM logs WHERE id = ?1")?
                .query(params![id])?,
        )?;

        // Fetch and cache tags for later lookup
        let tags_lookup: HashMap<i32, HashSet<String>> = tags_lookup_from_rows(
            self.connection
                .prepare("SELECT log_id, tag FROM log_tags WHERE log_id = ?1")?
                .query(params![id])?,
        )?;

        // Fetch and cache events for later lookup
        let events_lookup = events_lookup_from_rows(
            self.connection
                .prepare("SELECT log_id, timestamp, event FROM log_events WHERE log_id = ?1")?
                .query(params![id])?,
        )?;

        // Create work items from the cached data
        Ok(create_work_items(item_lookup, tags_lookup, events_lookup).pop())
    }

    fn find_items_by_status(&self, status: Status) -> Result<Vec<WorkItem>, Box<dyn Error>> {
        let item_lookup = tmp_item_lookup_from_rows(
            self.connection
                .prepare("SELECT id, description, status FROM logs WHERE status = ?1")?
                .query(params![format!("{}", status)])?,
        )?;

        // Fetch and cache tags for later lookup
        let tags_lookup: HashMap<i32, HashSet<String>> = tags_lookup_from_rows(
            self.connection
                .prepare(
                    "SELECT log_id, tag FROM log_tags WHERE log_id IN (\
                    SELECT id FROM logs WHERE status = ?1
                )",
                )?
                .query(params![format!("{}", status)])?,
        )?;

        // Fetch and cache events for later lookup
        let events_lookup = events_lookup_from_rows(
            self.connection
                .prepare(
                    "SELECT log_id, timestamp, event FROM log_events WHERE log_id IN (\
                    SELECT id FROM logs WHERE status = ?1
                )",
                )?
                .query(params![format!("{}", status)])?,
        )?;

        // Create work items from the cached data
        Ok(create_work_items(item_lookup, tags_lookup, events_lookup))
    }

    fn delete_item(&mut self, id: i32) -> Result<Option<WorkItem>, Box<dyn Error>> {
        let transaction = self.connection.transaction()?;

        // First and foremost find work item
        let item = {
            let item_lookup = tmp_item_lookup_from_rows(
                transaction
                    .prepare("SELECT id, description, status FROM logs WHERE id = ?1")?
                    .query(params![id])?,
            )?;

            // Fetch and cache tags for later lookup
            let tags_lookup: HashMap<i32, HashSet<String>> = tags_lookup_from_rows(
                transaction
                    .prepare("SELECT log_id, tag FROM log_tags WHERE log_id = ?1")?
                    .query(params![id])?,
            )?;

            // Fetch and cache events for later lookup
            let events_lookup = events_lookup_from_rows(
                transaction
                    .prepare("SELECT log_id, timestamp, event FROM log_events WHERE log_id = ?1")?
                    .query(params![id])?,
            )?;

            // Create work items from the cached data
            create_work_items(item_lookup, tags_lookup, events_lookup).pop()
        };

        // Delete from log_tags table first
        transaction.execute("DELETE FROM log_tags WHERE log_id = ?1", params![id])?;

        // Delete from log_events table second
        transaction.execute("DELETE FROM log_events WHERE log_id = ?1", params![id])?;

        // Delete from logs table third
        transaction.execute("DELETE FROM logs WHERE id = ?1", params![id])?;

        transaction.commit()?;

        Ok(item)
    }

    fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        let transaction = self.connection.transaction()?;

        // Clear the log_tags table
        transaction.execute("DELETE FROM log_tags", NO_PARAMS)?;

        // Clear the log_events table
        transaction.execute("DELETE FROM log_events", NO_PARAMS)?;

        // Clear the logs table
        transaction.execute("DELETE FROM logs", NO_PARAMS)?;

        transaction.commit()?;

        Ok(())
    }
}

/// Create final work items from the passed caches.
fn create_work_items(
    item_lookup: HashMap<i32, TmpWorkItem>,
    mut tags_lookup: HashMap<i32, HashSet<String>>,
    mut events_lookup: HashMap<i32, Vec<Event>>,
) -> Vec<WorkItem> {
    item_lookup
        .into_iter()
        .map(|(id, tmp_item)| {
            // Retrieve cached tags
            let tags = tags_lookup.remove(&id).unwrap_or(HashSet::new());

            // Retrieve cached events and sort them by their timestamp
            let mut events = events_lookup
                .remove(&id)
                .expect("Found no events for a work item, which must not be possible");
            events.sort_by_key(|e| e.timestamp());

            WorkItem::new_internal(
                tmp_item.id,
                tmp_item.description,
                tmp_item.status,
                tags,
                events,
            )
        })
        .collect()
}

/// Create a tmp work item lookup from the given logs table rows.
fn tmp_item_lookup_from_rows(mut rows: Rows) -> Result<HashMap<i32, TmpWorkItem>, Box<dyn Error>> {
    let mut item_lookup = HashMap::new();
    while let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;

        let description = row.get(1)?;

        let status_str: String = row.get(2)?;
        let status = Status::from_str(&status_str)
            .expect("Could not interpret Status string stored in the database");

        item_lookup.insert(
            id,
            TmpWorkItem {
                id,
                description,
                status,
            },
        );
    }

    Ok(item_lookup)
}

/// Create a tags lookup from the passed log_tags table rows.
fn tags_lookup_from_rows(mut rows: Rows) -> Result<HashMap<i32, HashSet<String>>, Box<dyn Error>> {
    let mut tags_lookup: HashMap<i32, HashSet<String>> = HashMap::new();
    while let Some(row) = rows.next()? {
        let log_id: i32 = row.get(0)?;
        let tag: String = row.get(1)?;

        match tags_lookup.entry(log_id) {
            Entry::Occupied(mut e) => {
                e.get_mut().insert(tag);
            }
            Entry::Vacant(e) => {
                let mut set = HashSet::new();
                set.insert(tag);
                e.insert(set);
            }
        };
    }

    Ok(tags_lookup)
}

/// Create an events lookup from the passed log_events table rows.
fn events_lookup_from_rows(mut rows: Rows) -> Result<HashMap<i32, Vec<Event>>, Box<dyn Error>> {
    let mut events_lookup: HashMap<i32, Vec<Event>> = HashMap::new();
    while let Some(row) = rows.next()? {
        let log_id: i32 = row.get(0)?;

        let timestamp: i64 = row.get(1)?;

        let event_str: String = row.get(2)?;
        let event_type =
            EventType::from_str(&event_str).expect("EventType string could not be interpreted");

        match events_lookup.entry(log_id) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(Event::new(event_type, timestamp));
            }
            Entry::Vacant(e) => {
                e.insert(vec![Event::new(event_type, timestamp)]);
            }
        };
    }

    Ok(events_lookup)
}

/// Delete all tags for the work item with the given ID.
fn delete_tags(transaction: &Transaction, id: i32) -> Result<(), Box<dyn Error>> {
    transaction.execute("DELETE FROM log_tags WHERE log_id = ?1", params![id])?;

    Ok(())
}

/// Insert all the given tags for the work item with the passed ID.
fn insert_tags(transaction: &Transaction, id: i32, tags: &[String]) -> Result<(), Box<dyn Error>> {
    for tag in tags {
        transaction.execute(
            "INSERT INTO log_tags (log_id, tag) VALUES (?1, ?2)",
            params![id, tag],
        )?;
    }

    Ok(())
}

/// Delete all events for the work item with the given ID.
fn delete_events(transaction: &Transaction, id: i32) -> Result<(), Box<dyn Error>> {
    transaction.execute("DELETE FROM log_events WHERE log_id = ?1", params![id])?;

    Ok(())
}

/// Insert all the given events for the work item with the passed ID.
fn insert_events(
    transaction: &Transaction,
    id: i32,
    events: &[Event],
) -> Result<(), Box<dyn Error>> {
    for event in events {
        transaction.execute(
            "INSERT INTO log_events (log_id, timestamp, event) VALUES (?1, ?2, ?3)",
            params![id, event.timestamp(), format!("{}", event.event_type())],
        )?;
    }

    Ok(())
}
