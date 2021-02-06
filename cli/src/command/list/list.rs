use crate::command::command::Command;
use crate::util;
use cmd_args::{arg, option, Group};
use colorful::Colorful;
use std::collections::HashMap;
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
    entries.sort_by_key(|v| i64::max_value() - v.timestamp());

    let mut last_date_option: Option<chrono::Date<chrono::Utc>> = None;
    for entry in &entries {
        let date_time: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_utc(
            chrono::NaiveDateTime::from_timestamp(entry.timestamp() / 1000, 0),
            chrono::Utc,
        );

        let display_date = match last_date_option {
            Some(last_date) => date_time.date().sub(last_date).num_days().abs() >= 1,
            None => true,
        };

        if display_date {
            println!();
            println!(
                "# {}",
                date_time.format("%A - %d. %B %Y").to_string().underlined()
            );
            println!();
        }

        let tags_formatted: Vec<_> = entry.tags().iter().map(|s| format!("#{}", s)).collect();

        println!(
            "• {} [{}] {} - {} ({})",
            format!(
                "#{}",
                entry
                    .id()
                    .expect("Work item must have an ID at this point!")
            )
            .color(colorful::Color::DodgerBlue3),
            date_time
                .format("%H:%M")
                .to_string()
                .color(colorful::Color::DeepPink1a),
            entry.description(),
            util::format_duration(entry.time_taken() as u32).color(colorful::Color::Orange1),
            tags_formatted
                .join(", ")
                .color(colorful::Color::DarkSlateGray1)
        );

        last_date_option = Some(date_time.date());
    }

    println!();
}

/// Convert the passed filter keyword ("today", "yesterday", "2020-02-02")
/// to a time range of timestamps.
fn filter_keyword_to_time_range(keyword: &str) -> (i64, i64) {
    match keyword {
        "today" => {
            let today = chrono::Utc::today();
            let from_timestamp = today.and_hms(0, 0, 0).timestamp_millis();
            let to_timestamp = today.succ().and_hms(0, 0, 0).timestamp_millis();

            return (from_timestamp, to_timestamp);
        }
        "yesterday" => {
            let yesterday = chrono::Utc::today().pred();
            let from_timestamp = yesterday.and_hms(0, 0, 0).timestamp_millis();
            let to_timestamp = yesterday.succ().and_hms(0, 0, 0).timestamp_millis();

            return (from_timestamp, to_timestamp);
        }
        str => {
            let date = chrono::NaiveDate::parse_from_str(str, "%Y-%m-%d")
                .expect("Expected date format to be in form 'YYYY-MM-dd'");
            let from_timestamp = date.and_hms(0, 0, 0).timestamp_millis();
            let to_timestamp = date.succ().and_hms(0, 0, 0).timestamp_millis();

            return (from_timestamp, to_timestamp);
        }
    }
}
