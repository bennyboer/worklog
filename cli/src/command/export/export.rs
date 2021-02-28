use crate::command::command::Command;
use crate::command::list;
use cmd_args::{arg, option, Group};
use persistence::calc::event::EventType;
use persistence::calc::WorkItem;
use std::collections::HashMap;
use std::fs;

/// Command used to export work items.
pub struct ExportCommand {}

impl Command for ExportCommand {
    fn build(&self) -> Group {
        Group::new(
            Box::new(|args, options| execute(args, options)),
            "Export work log entries",
        )
        .add_argument(arg::Descriptor::new(
            arg::Type::Str,
            "Export type (e. g. 'markdown')",
        ))
        .add_option(option::Descriptor::new(
            "path",
            option::Type::Str {
                default: String::from("log_export.md"),
            },
            "Path of the file to export to",
        ))
        .add_option(option::Descriptor::new(
            "filter",
            option::Type::Str {
                default: String::from("today"),
            },
            "Filter by a date ('today' (default), 'yesterday', '2020-02-20' (yyyy-MM-dd))",
        ))
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        None
    }

    fn name(&self) -> &str {
        "export"
    }
}

/// Execute the export command.
fn execute(args: &Vec<arg::Value>, options: &HashMap<&str, option::Value>) {
    let export_type = args[0].str().unwrap();

    if export_type.trim().to_lowercase() == "markdown" {
        export_to_markdown(
            options.get("path").unwrap().str().unwrap(),
            options
                .get("filter")
                .unwrap()
                .str()
                .map_or(String::from("today"), |v| v.to_owned()),
        );
    } else {
        panic!("Export type '{}' currently not supported", export_type);
    }
}

/// Export to a markdown file with the given file path.
fn export_to_markdown(file_path: &str, filter: String) {
    let (from_timestamp, to_timestamp) = list::filter_keyword_to_time_range(&filter[..]);
    let items = persistence::find_items_by_timerange(from_timestamp, to_timestamp).unwrap();

    let first_item = items.first().unwrap();
    let date_time = shared::time::get_local_date_time(first_item.created_timestamp());

    let mut data = String::new();

    data.push_str(&format!(
        "# Report for {} the {}\n\n",
        date_time.format("%A").to_string(),
        date_time.format("%Y-%m-%d").to_string()
    ));

    data.push_str("## Statistics\n\n");

    let total_work_time = {
        let item_refs: Vec<&WorkItem> = items.iter().collect();
        let total_work_time_ms = list::calculate_total_work_time(&item_refs);

        shared::time::format_duration((total_work_time_ms / 1000) as u32)
    };
    let start_time =
        shared::time::get_local_date_time(find_earliest_work_item(&items).created_timestamp())
            .format("%H:%M")
            .to_string();
    let end_time =
        shared::time::get_local_date_time(find_latest_work_item(&items).created_timestamp())
            .format("%H:%M")
            .to_string();

    data.push_str(&format!(
        "\
| Total time worked | Started working | Finished working |
| ----------------- | --------------- | ---------------- |
| {} | {} | {} |\n\n",
        total_work_time, start_time, end_time
    ));

    data.push_str("## Work items\n\n");

    for item in &items {
        data.push_str(&format!(
            "- {}. Took `{}` ({}). Tags: *{}*.\n",
            item.description(),
            shared::time::format_duration((item.time_taken() / 1000) as u32),
            format_event_timeline(item),
            item.tags().join(", ")
        ));
    }

    fs::write(file_path, data).expect("Unable to write export file");
}

fn format_event_timeline(item: &WorkItem) -> String {
    let mut result = Vec::new();

    let mut start_time = None;
    for event in item.events() {
        match event.event_type() {
            EventType::Started | EventType::Continued => {
                start_time = Some(shared::time::get_local_date_time(event.timestamp()))
            }
            EventType::Finished | EventType::Paused => result.push(format!(
                "{} - {}",
                start_time.unwrap().format("%H:%M"),
                shared::time::get_local_date_time(event.timestamp()).format("%H:%M")
            )),
        };
    }

    result.join(", ")
}

/// Find the latest work item among the list of passed work item references.
fn find_latest_work_item(items: &[WorkItem]) -> &WorkItem {
    let mut lastest_ts: i64 = items.first().unwrap().events().last().unwrap().timestamp();
    let mut latest_item = items.first().unwrap();
    for item in &items[1..] {
        if item.events().last().unwrap().timestamp() > lastest_ts {
            lastest_ts = item.events().last().unwrap().timestamp();
            latest_item = item;
        }
    }

    latest_item
}

/// Find the earliest work item among the list of passed work item references.
fn find_earliest_work_item(items: &[WorkItem]) -> &WorkItem {
    let mut earliest_ts: i64 = items.first().unwrap().created_timestamp();
    let mut earliest_item = items.first().unwrap();
    for item in &items[1..] {
        if item.created_timestamp() < earliest_ts {
            earliest_ts = item.created_timestamp();
            earliest_item = item;
        }
    }

    earliest_item
}
