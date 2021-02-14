use std::collections::HashMap;

use cmd_args::{arg, option, Group};

use crate::command::command::Command;

/// Command used to delete a work item.
pub struct DeleteCommand {}

impl Command for DeleteCommand {
    fn build(&self) -> Group {
        Group::new(
            Box::new(|args, options| execute(args, options)),
            "Delete a work item",
        )
        .add_argument(arg::Descriptor::new(
            arg::Type::Int,
            "ID of the work item to delete",
        ))
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        Some(vec!["remove", "r", "d"])
    }

    fn name(&self) -> &str {
        "delete"
    }
}

/// Execute the delete command.
fn execute(args: &Vec<arg::Value>, _options: &HashMap<&str, option::Value>) {
    let id = args[0].int().expect("Expected to have an ID supplied");

    match persistence::delete_item(id) {
        Ok(item) => match item {
            Some(_) => println!("Work item with ID {} has been deleted.", id),
            None => println!("There is no work item with ID {}.", id),
        },
        Err(e) => println!(
            "An error occurred trying to delete work item with ID {}. Error: '{}'.",
            id, e
        ),
    };
}
