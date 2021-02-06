use crate::command::command::Command;
use cmd_args::{arg, option, Group};
use std::collections::HashMap;

/// Command used to start a work item.
pub struct StartCommand {}

impl Command for StartCommand {
    fn build(&self) -> Group {
        Group::new(
            Box::new(|args, options| execute(args, options)),
            "Start working on a work item",
        )
        .add_argument(arg::Descriptor::new(
            arg::Type::Str,
            "Description of the work item to start working on",
        ))
        .add_argument(arg::Descriptor::new(arg::Type::Str, "Tags"))
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        None
    }

    fn name(&self) -> &str {
        "start"
    }
}

/// Execute the start command.
fn execute(_args: &Vec<arg::Value>, _options: &HashMap<&str, option::Value>) {
    // TODO
}
