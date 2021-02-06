use crate::command::command::Command;
use cmd_args::{arg, option, Group};
use std::collections::HashMap;

/// Command used to pause working on an in progress work item.
pub struct PauseCommand {}

impl Command for PauseCommand {
    fn build(&self) -> Group {
        Group::new(
            Box::new(|args, options| execute(args, options)),
            "Pause working on an in progress work item",
        )
        .add_argument(arg::Descriptor::new(
            arg::Type::Int,
            "ID of the work item to pause working on",
        ))
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        None
    }

    fn name(&self) -> &str {
        "pause"
    }
}

/// Execute the pause command.
fn execute(_args: &Vec<arg::Value>, _options: &HashMap<&str, option::Value>) {
    // TODO
}
