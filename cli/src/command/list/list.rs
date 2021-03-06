use std::collections::HashMap;

use cmd_args::{arg, option, Group};
use colorful::Colorful;

use persistence::calc::{Status, WorkItem};

use crate::command::command::Command;
use std::ops::Sub;

/// Command used to list work items.
pub struct ListCommand {}

impl Command for ListCommand {
    fn build(&self) -> Group {
        Group::new(
            Box::new(|args, options| execute(args, options)),
            "List work items",
        )
        .add_option(option::Descriptor::new(
            "all",
            option::Type::Bool { default: false },
            "Show all available log entries",
        ))
        .add_option(option::Descriptor::new(
            "filter",
            option::Type::Str {
                default: String::from("today"),
            },
            "Filter by a date ('today' (default), 'yesterday', '2020-02-20' (yyyy-MM-dd)) or work item ID",
        ))
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        Some(vec!["ls"])
    }

    fn name(&self) -> &str {
        "list"
    }
}

/// Execute the list command.
fn execute(_args: &Vec<arg::Value>, options: &HashMap<&str, option::Value>) {
    let all: bool = options.get("all").map_or(false, |v| v.bool().unwrap());

    let mut entries = match all {
        true => persistence::list_items().unwrap(),
        false => {
            let filter: &str = options.get("filter").map_or("today", |v| v.str().unwrap());

            // Check if filter string is a work item ID
            match filter.parse::<i32>() {
                Ok(id) => persistence::find_item_by_id(id)
                    .unwrap()
                    .map_or(Vec::new(), |v| vec![v]),
                Err(_) => {
                    // Filter string is not an work item ID but a date!
                    let (from_timestamp, to_timestamp) = filter_keyword_to_time_range(filter);

                    persistence::find_items_by_timerange(from_timestamp, to_timestamp).unwrap()
                }
            }
        }
    };

    let found_str: String = format!("| Found {} log entries |", entries.len());

    println!(" {} ", "-".repeat(found_str.len() - 2));
    println!("{}", found_str);
    println!(" {} ", "-".repeat(found_str.len() - 2));

    // Sort entries by their timestamp (newest come first).
    entries.sort_by_key(|v| i64::max_value() - v.created_timestamp());

    // First collect all items per day
    let mut items_per_day = Vec::new();
    items_per_day.push(Vec::new());
    let mut last_date_option: Option<chrono::Date<_>> = None;
    for item in &entries {
        let date_time = shared::time::get_local_date_time(item.created_timestamp());

        let is_another_day = match last_date_option {
            Some(last_date) => date_time.date().sub(last_date).num_days().abs() >= 1,
            None => false,
        };

        if is_another_day {
            items_per_day.push(Vec::new());
        }

        let current_day_items = items_per_day.last_mut().unwrap();
        current_day_items.push(item);

        last_date_option = Some(date_time.date());
    }

    // Print work items for each day
    for items in items_per_day {
        if !items.is_empty() {
            print_date_header(&items);

            for item in items {
                println!("  • {}", format_item(item));
            }
        }
    }

    println!();
}

/// Print the header for a new date.
fn print_date_header(items: &[&WorkItem]) {
    let first = *items.first().unwrap();
    let date_time = shared::time::get_local_date_time(first.created_timestamp());

    println!();
    println!(
        "{}",
        format!(
            "# {} ({})",
            date_time.format("%A - %d. %B %Y"),
            shared::time::format_duration((calculate_total_work_time(items) / 1000) as u32)
        )
        .underlined()
    );
    println!();
}

/// Calculate the total work time of the passed items.
pub(crate) fn calculate_total_work_time(items: &[&WorkItem]) -> i64 {
    let mut time_events: Vec<shared::calc::TimeEvent> = items
        .iter()
        .flat_map(|i| {
            i.events()
                .iter()
                .map(|e| shared::calc::TimeEvent::new(e.is_start(), e.timestamp()))
        })
        .collect();

    for item in items {
        if let Status::InProgress = item.status() {
            // Add dummy end time event to calculate the correct time
            time_events.push(shared::calc::TimeEvent::new(
                false,
                chrono::Utc::now().timestamp_millis(),
            ));
        }
    }

    shared::calc::calculate_unique_total_time(&mut time_events)
}

/// Format a work item.
fn format_item(item: &WorkItem) -> String {
    let id_str = format!(
        "#{}",
        item.id().expect("Work item must have an ID at this point!")
    )
    .color(colorful::Color::DodgerBlue3);

    let time_str = shared::time::get_local_date_time(item.created_timestamp())
        .format("%H:%M")
        .to_string()
        .color(colorful::Color::DeepPink1a);

    let description = item.description();

    let duration_str = shared::time::format_duration((item.time_taken() / 1000) as u32)
        .color(colorful::Color::Orange1);
    let status_str = match item.status() {
        Status::Done => duration_str,
        Status::InProgress => {
            format!("IN PROGRESS ({})", duration_str).color(colorful::Color::GreenYellow)
        }
        Status::Paused => format!("PAUSED ({})", duration_str).color(colorful::Color::Red),
    };

    let tags_formatted: Vec<_> = item.tags().iter().map(|s| format!("#{}", s)).collect();
    let tags_str = tags_formatted
        .join(", ")
        .color(colorful::Color::DarkSlateGray1);

    format!(
        "{} [{}] {} - {} ({})",
        id_str, time_str, description, status_str, tags_str
    )
}

/// Convert the passed filter keyword ("today", "yesterday", "2020-02-02")
/// to a time range of timestamps.
pub(crate) fn filter_keyword_to_time_range(keyword: &str) -> (i64, i64) {
    match keyword {
        "today" => {
            let today = chrono::Utc::today();
            let from_timestamp = today.and_hms(0, 0, 0).timestamp_millis();
            let to_timestamp = today.succ().and_hms(0, 0, 0).timestamp_millis();

            (from_timestamp, to_timestamp)
        }
        "yesterday" => {
            let yesterday = chrono::Utc::today().pred();
            let from_timestamp = yesterday.and_hms(0, 0, 0).timestamp_millis();
            let to_timestamp = yesterday.succ().and_hms(0, 0, 0).timestamp_millis();

            (from_timestamp, to_timestamp)
        }
        str => {
            let date = chrono::NaiveDate::parse_from_str(str, "%Y-%m-%d")
                .expect("Expected date format to be in form 'YYYY-MM-dd'");
            let from_timestamp = date.and_hms(0, 0, 0).timestamp_millis();
            let to_timestamp = date.succ().and_hms(0, 0, 0).timestamp_millis();

            (from_timestamp, to_timestamp)
        }
    }
}
