use crate::command::command::Command;
use cmd_args::{arg, option, Group};
use std::collections::HashMap;

/// Command used to stop an in progress work item.
pub struct StopCommand {}

impl Command for StopCommand {
    fn build(&self) -> Group {
        Group::new(
            Box::new(|args, options| execute(args, options)),
            "Stop working on a work item in progress",
        )
        .add_argument(arg::Descriptor::new(
            arg::Type::Int,
            "ID of the work item to stop working on",
        ))
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        None
    }

    fn name(&self) -> &str {
        "stop"
    }
}

/// Execute the stop command.
fn execute(_args: &Vec<arg::Value>, _options: &HashMap<&str, option::Value>) {
    // TODO
}
