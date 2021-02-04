mod util;

use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, Utc};
use cmd_args::arg::Value;
use cmd_args::{arg, option, parser, Group};
use colorful::{Color, Colorful};
use persistence;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::ops::Sub;
use std::{fs, process};

fn main() {
    let mut group = Group::new(
        Box::new(|_args, _options| {
            println!("### Incorrect usage ###");
            println!("Pass '--help' to see all available options.");
            process::exit(1);
        }),
        "Tool to log your work",
    );

    // "log" sub-command
    group = group.add_child(
        "log",
        Some(vec!["l"]),
        Group::new(
            Box::new(|args, options| {
                log(args, options);
            }),
            "Log a new work entry",
        )
        .add_argument(arg::Descriptor::new(
            arg::Type::Str,
            "Description of the work done",
        ))
        .add_argument(arg::Descriptor::new(arg::Type::Str, "Tags"))
        .add_argument(arg::Descriptor::new(
            arg::Type::Str,
            "Time spent on the task",
        )),
    );

    // "list" sub-command
    group = group.add_child(
        "list",
        Some(vec!["ls"]),
        Group::new(
            Box::new(|args, options| {
                list(args, options);
            }),
            "List all work entries",
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
            "Filter by a date ('today' (default), 'yesterday', '2020-02-20' (yyyy-MM-dd))",
        )),
    );

    // "export" sub-command
    group = group.add_child(
        "export",
        None,
        Group::new(
            Box::new(|args, options| {
                export(args, options);
            }),
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
        )),
    );

    parser::parse(group, None).unwrap();
}

fn log(args: &[Value], _options: &HashMap<&str, option::Value>) {
    let description = args[0].str().unwrap();
    let tags_str = args[1].str().unwrap();
    let time_taken_str = args[2].str().unwrap();

    let tags: Vec<String> = tags_str.split(",").map(|s| s.trim().to_owned()).collect();
    let time_taken = util::parse_duration(time_taken_str).unwrap();

    let entry = persistence::entry::LogEntry::new(
        description.to_owned(),
        HashSet::from_iter(tags.into_iter()),
        time_taken,
    );

    persistence::log_entry(entry).unwrap();
}

fn list(_args: &[Value], options: &HashMap<&str, option::Value>) {
    let all: bool = options.get("all").map_or(false, |v| v.bool().unwrap());

    let mut entries = match all {
        true => persistence::list_entries().unwrap(),
        false => {
            let filter: &str = options.get("filter").map_or("today", |v| v.str().unwrap());

            let (from_timestamp, to_timestamp) = filter_keyword_to_time_range(filter);

            persistence::filter_entries(from_timestamp, to_timestamp).unwrap()
        }
    };

    let found_str: String = format!("| Found {} log entries |", entries.len());

    println!(" {} ", "-".repeat(found_str.len() - 2));
    println!("{}", found_str);
    println!(" {} ", "-".repeat(found_str.len() - 2));

    // Sort entries by their timestamp (newest come first).
    entries.sort_by_key(|v| i64::max_value() - v.timestamp());

    let mut last_date_option: Option<Date<Utc>> = None;
    for entry in &entries {
        let date_time: DateTime<Utc> = DateTime::from_utc(
            NaiveDateTime::from_timestamp(entry.timestamp() / 1000, 0),
            Utc,
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
            "• [{}] {} - {} ({})",
            date_time
                .format("%H:%M")
                .to_string()
                .color(Color::DeepPink1a),
            entry.description(),
            util::format_duration(entry.time_taken() as u32).color(Color::Orange1),
            tags_formatted.join(", ").color(Color::DarkSlateGray1)
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
            let today = Utc::today();
            let from_timestamp = today.and_hms(0, 0, 0).timestamp_millis();
            let to_timestamp = today.succ().and_hms(0, 0, 0).timestamp_millis();

            return (from_timestamp, to_timestamp);
        }
        "yesterday" => {
            let yesterday = Utc::today().pred();
            let from_timestamp = yesterday.and_hms(0, 0, 0).timestamp_millis();
            let to_timestamp = yesterday.succ().and_hms(0, 0, 0).timestamp_millis();

            return (from_timestamp, to_timestamp);
        }
        str => {
            let date = NaiveDate::parse_from_str(str, "%Y-%m-%d")
                .expect("Expected date format to be in form 'YYYY-MM-dd'");
            let from_timestamp = date.and_hms(0, 0, 0).timestamp_millis();
            let to_timestamp = date.succ().and_hms(0, 0, 0).timestamp_millis();

            return (from_timestamp, to_timestamp);
        }
    }
}

fn export(args: &[Value], options: &HashMap<&str, option::Value>) {
    let export_type = args[0].str().unwrap();

    if export_type.trim().to_lowercase() == "markdown" {
        export_to_markdown(options.get("path").unwrap().str().unwrap());
    } else {
        panic!("Export type '{}' currently not supported", export_type);
    }
}

fn export_to_markdown(file_path: &str) {
    let mut data = String::new();

    let entries = persistence::list_entries().unwrap();
    for entry in &entries {
        data.push_str(&format!(
            "- {} ({}), took {} minutes\n",
            entry.description(),
            entry.tags().join(", "),
            entry.time_taken() / 60
        ));
    }

    fs::write(file_path, data).expect("Unable to write export file");
}
