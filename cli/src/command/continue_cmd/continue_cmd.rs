use std::collections::HashMap;

use cmd_args::{arg, option, Group};

use persistence::work_item::Status;

use crate::command::command::Command;

/// Command used to continue working on an in progress work item.
pub struct ContinueCommand {}

impl Command for ContinueCommand {
    fn build(&self) -> Group {
        Group::new(
            Box::new(|args, options| execute(args, options)),
            "Continue working on an in progress work item",
        )
        .add_argument(arg::Descriptor::new(
            arg::Type::Int,
            "ID of the work item to continue working on",
        ))
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        Some(vec!["c"])
    }

    fn name(&self) -> &str {
        "continue"
    }
}

/// Execute the continue command.
fn execute(args: &Vec<arg::Value>, _options: &HashMap<&str, option::Value>) {
    let id = args[0]
        .int()
        .expect("Expected an ID of a work item as first argument");

    match persistence::find_item_by_id(id) {
        Ok(option) => match option {
            Some(mut item) => {
                if let Status::Paused = item.status() {
                    item.continue_working().unwrap();

                    match persistence::update_items(vec![&item]) {
                        Ok(_) => println!("Continued work item with ID {}.", id),
                        Err(e) => println!(
                            "Failed to continue work item with ID {}. Error: '{}'",
                            id, e
                        ),
                    };
                } else {
                    println!("Work item with ID {} is currently not paused and thus cannot be continued working on.", id);
                }
            }
            None => println!("Could not find work item with ID {}.", id),
        },
        Err(e) => println!(
            "An error occurred while trying to lookup the work item with ID {}. Error: '{}'",
            id, e
        ),
    };
}
