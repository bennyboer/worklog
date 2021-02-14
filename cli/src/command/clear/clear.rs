use std::collections::HashMap;

use cmd_args::{arg, option, Group};

use crate::command::command::Command;

/// Command used to clear the database (remove all work items).
pub struct ClearCommand {}

impl Command for ClearCommand {
    fn build(&self) -> Group {
        Group::new(
            Box::new(|args, options| execute(args, options)),
            "Clear the database (Remove all work items)",
        )
        .add_option(option::Descriptor::new(
            "ack",
            option::Type::Bool { default: false },
            "Acknowledge the database clearing",
        ))
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        None
    }

    fn name(&self) -> &str {
        "clear"
    }
}

/// Execute the clear command.
fn execute(_args: &Vec<arg::Value>, options: &HashMap<&str, option::Value>) {
    let acknowlegement = options.get("ack").unwrap().bool().unwrap();

    if acknowlegement {
        match persistence::clear() {
            Ok(_) => println!("Cleared the database (Removed all work items)."),
            Err(e) => println!("Could not clear the database. Error: '{}'.", e),
        }
    } else {
        println!("Do you really want to clear the database (Remove all work items)?");
        println!("Please acknowledge the operation by re-entering the clear command followed by the --ack flag");
    }
}
