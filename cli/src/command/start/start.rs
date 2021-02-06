use crate::command::command::Command;
use cmd_args::{arg, option, Group};
use colorful::Colorful;
use persistence::work_item::Status;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

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
fn execute(args: &Vec<arg::Value>, _options: &HashMap<&str, option::Value>) {
    let description = args[0].str().unwrap();
    let tags_str = args[1].str().unwrap();

    let tags: Vec<String> = tags_str.split(",").map(|s| s.trim().to_owned()).collect();

    let entry = persistence::work_item::WorkItem::new(
        description.to_owned(),
        HashSet::from_iter(tags.into_iter()),
        0,
        Status::InProgress,
    );

    let new_id = persistence::log_item(entry).unwrap();

    println!(
        "Create work item with ID {}",
        format!("#{}", new_id).color(colorful::Color::DodgerBlue3)
    );
}
