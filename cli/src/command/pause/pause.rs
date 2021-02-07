use std::collections::HashMap;

use cmd_args::{arg, option, Group};

use persistence::work_item::{Status, WorkItem};

use crate::command::command::Command;

/// Command used to pause working on an in progress work item.
pub struct PauseCommand {}

impl Command for PauseCommand {
    fn build(&self) -> Group {
        Group::new(
            Box::new(|args, options| execute(args, options)),
            "Pause working on an in progress work item",
        )
        .add_argument(arg::Descriptor::new(
            arg::Type::Str,
            "ID of the work item to pause working on or 'all' to pause all work items",
        ))
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        Some(vec!["p"])
    }

    fn name(&self) -> &str {
        "pause"
    }
}

/// Execute the pause command.
fn execute(args: &Vec<arg::Value>, _options: &HashMap<&str, option::Value>) {
    let value = args[0].str().expect("Expected to have one argument");

    match value.parse::<i32>() {
        Ok(id) => {
            pause_work_item_by_id(id);
        }
        Err(_) => {
            // Check if value is "all" to pause all work items in progress
            if value == "all" {
                pause_all_work_items_in_progress();

                println!("Paused all work items in progress.");
            }
        }
    }
}

fn pause_work_item_by_id(id: i32) {
    match persistence::find_item_by_id(id).unwrap() {
        Some(mut item) => {
            if let Status::InProgress = item.status() {
                pause_work_item(&mut item);

                match persistence::update_items(vec![&item]) {
                    Ok(_) => println!("Paused work item with ID {}.", id),
                    Err(e) => println!("Failed to update work item with ID {}. Error: '{}'", id, e),
                };
            } else {
                println!(
                    "Work item with ID {} is currently not in progress and thus cannot be paused.",
                    id
                );
            }
        }
        None => {
            println!("Could not find work item with ID {}.", id);
        }
    }
}

pub(crate) fn pause_all_work_items_in_progress() {
    let mut result = persistence::find_items_by_status(Status::InProgress).unwrap();

    let mut to_update = Vec::new();
    for item in result.iter_mut() {
        pause_work_item(item);

        to_update.push(&*item);
    }

    persistence::update_items(to_update).unwrap();
}

pub(crate) fn pause_work_item(item: &mut WorkItem) {
    item.set_status(Status::Paused);

    let pause_timestamp = chrono::Utc::now().timestamp_millis();

    // Set time_taken to the current time
    let timer_timestamp = item
        .timer_timestamp()
        .expect("Timer timestamp must be present at this point!");
    let time_taken = pause_timestamp - timer_timestamp;

    item.set_time_taken(item.time_taken() + time_taken);

    // Update timer_timestamp to the pause timestamp
    item.set_timer_timestamp(Some(pause_timestamp));
}
