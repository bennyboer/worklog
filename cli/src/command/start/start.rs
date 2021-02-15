use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use cmd_args::{arg, option, Group};
use colorful::Colorful;

use persistence::work_item::Status;

use crate::command::command::Command;
use crate::command::{finish, pause};

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
        .add_option(option::Descriptor::new(
            "finish",
            option::Type::Bool { default: false },
            "Finish all work items currently in progress",
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
    let finish_work_items_in_progress = options.get("finish").unwrap().bool().unwrap();

    // If both --pause and --finish are specified we are finishing all items!

    // Stopping in progress work items first
    if finish_work_items_in_progress || pause_work_items_in_progress {
        pause::pause_all_work_items_in_progress();
    }

    // When --finish specified -> Finish all paused work items
    if finish_work_items_in_progress {
        finish::finish_all_paused_work_items();
    }

    let item = persistence::work_item::WorkItem::new(
        description.to_owned(),
        Status::InProgress,
        HashSet::from_iter(tags.into_iter()),
    );

    let new_id = persistence::log_item(item).unwrap();

    println!(
        "Started working on work item with ID {}.",
        format!("#{}", new_id).color(colorful::Color::DodgerBlue3)
    );
}
