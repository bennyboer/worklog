use std::collections::HashMap;

use cmd_args::{arg, option, Group};

use persistence::work_item::{Status, WorkItem};

use crate::command::command::Command;
use crate::command::pause;

/// Command used to finish an in progress work item.
pub struct FinishCommand {}

impl Command for FinishCommand {
    fn build(&self) -> Group {
        Group::new(
            Box::new(|args, options| execute(args, options)),
            "Finish working on a work item in progress",
        )
        .add_argument(arg::Descriptor::new(
            arg::Type::Int,
            "ID of the work item to finish working on",
        ))
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        Some(vec!["f"])
    }

    fn name(&self) -> &str {
        "finish"
    }
}

/// Execute the finish command.
fn execute(args: &Vec<arg::Value>, _options: &HashMap<&str, option::Value>) {
    let id = args[0]
        .int()
        .expect("Expected first argument to be a work item ID");

    match persistence::find_item_by_id(id).unwrap() {
        Some(mut item) => {
            if let Status::Done = item.status() {
                println!("Work item with ID {} is already finished.", id);
            } else {
                if let Status::InProgress = item.status() {
                    // Pause work item first!
                    pause::pause_work_item(&mut item);
                }

                finish_work_item(&mut item);

                match persistence::update_items(vec![&item]) {
                    Ok(_) => println!("Finished work item with ID {}.", id),
                    Err(e) => println!(
                        "Failed to update work item with ID {}. Error: '{}'",
                        item.id().unwrap(),
                        e
                    ),
                };
            }
        }
        None => {
            println!("Could not find work item with ID {}.", id);
        }
    }
}

fn finish_work_item(item: &mut WorkItem) {
    item.set_status(Status::Done);

    // Set timer timestamp to None
    item.set_timer_timestamp(None);
}
