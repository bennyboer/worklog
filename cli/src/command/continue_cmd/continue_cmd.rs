use crate::command::command::Command;
use cmd_args::{arg, option, Group};
use std::collections::HashMap;

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
        None
    }

    fn name(&self) -> &str {
        "continue"
    }
}

/// Execute the continue command.
fn execute(_args: &Vec<arg::Value>, _options: &HashMap<&str, option::Value>) {
    // TODO
}
