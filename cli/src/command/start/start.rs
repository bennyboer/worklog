use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use cmd_args::{arg, option, Group};
use colorful::Colorful;

use persistence::work_item::Status;

use crate::command::command::Command;
use crate::command::pause;

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
        .add_option(option::Descriptor::new(
            "pause",
            option::Type::Bool { default: false },
            "Pause all work items currently in progress",
        ))
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        Some(vec!["s"])
    }

    fn name(&self) -> &str {
        "start"
    }
}

/// Execute the start command.
fn execute(args: &Vec<arg::Value>, options: &HashMap<&str, option::Value>) {
    let description = args[0].str().unwrap();
    let tags_str = args[1].str().unwrap();

    let tags: Vec<String> = tags_str.split(",").map(|s| s.trim().to_owned()).collect();

    let pause_work_items_in_progress = options.get("pause").unwrap().bool().unwrap();

    // Stopping in progress work items first
    if pause_work_items_in_progress {
        pause::pause_all_work_items_in_progress();
    }

    let mut item = persistence::work_item::WorkItem::new(
        description.to_owned(),
        HashSet::from_iter(tags.into_iter()),
        0,
        Status::InProgress,
    );
    item.set_timer_timestamp(Some(item.timestamp()));

    let new_id = persistence::log_item(item).unwrap();

    println!(
        "Started working on work item with ID {}.",
        format!("#{}", new_id).color(colorful::Color::DodgerBlue3)
    );
}
