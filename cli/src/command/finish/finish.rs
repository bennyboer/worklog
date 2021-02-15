use std::collections::HashMap;

use cmd_args::{arg, option, Group};

use persistence::work_item::Status;

use crate::command::command::Command;

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
                item.finish_working(None).unwrap();

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

pub(crate) fn finish_all_paused_work_items() {
    let mut result = persistence::find_items_by_status(Status::Paused).unwrap();

    let mut to_update = Vec::new();
    for item in result.iter_mut() {
        item.finish_working(None).unwrap();

        to_update.push(&*item);
    }

    persistence::update_items(to_update).unwrap();
}
