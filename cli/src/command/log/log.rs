use crate::command::command::Command;
use cmd_args::{arg, option, Group};
use colorful::Colorful;
use persistence::calc::event::{Event, EventType};
use persistence::calc::Status;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

/// Command used to log work items directly.
pub struct LogCommand {}

impl Command for LogCommand {
    fn build(&self) -> Group {
        Group::new(
            Box::new(|args, options| execute(args, options)),
            "Log an already done work item",
        )
        .add_argument(arg::Descriptor::new(
            arg::Type::Str,
            "Description of the work done",
        ))
        .add_argument(arg::Descriptor::new(arg::Type::Str, "Tags"))
        .add_argument(arg::Descriptor::new(
            arg::Type::Str,
            "Time spent on the task (Format like '2h 3m 12s', '45m' or '1h 15m')",
        ))
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        Some(vec!["l"])
    }

    fn name(&self) -> &str {
        "log"
    }
}

/// Execute the log command.
fn execute(args: &Vec<arg::Value>, _options: &HashMap<&str, option::Value>) {
    let description = args[0].str().unwrap();
    let tags_str = args[1].str().unwrap();
    let time_taken_str = args[2].str().unwrap();

    let tags: Vec<String> = tags_str.split(",").map(|s| s.trim().to_owned()).collect();
    let time_taken_ms = shared::time::parse_duration(time_taken_str).unwrap() as i64 * 1000;

    let current_timestamp_ms = chrono::Utc::now().timestamp_millis();
    let item = persistence::calc::WorkItem::new_internal(
        -1,
        description.to_owned(),
        Status::Done,
        HashSet::from_iter(tags.into_iter()),
        vec![
            Event::new(EventType::Started, current_timestamp_ms - time_taken_ms),
            Event::new(EventType::Finished, current_timestamp_ms),
        ],
    );

    let new_id = persistence::log_item(item).unwrap();

    println!(
        "Create work item with ID {}.",
        format!("#{}", new_id).color(colorful::Color::DodgerBlue3)
    );
}
