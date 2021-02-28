use crate::command::command::Command;
use cmd_args::{arg, option, Group};
use colorful::Colorful;
use persistence::calc::WorkItem;
use std::collections::HashMap;

/// Command used to show details about a work item.
pub struct ShowCommand {}

impl Command for ShowCommand {
    fn build(&self) -> Group {
        Group::new(
            Box::new(|args, options| execute(args, options)),
            "Show details about a work item",
        )
        .add_argument(arg::Descriptor::new(
            arg::Type::Int,
            "ID of the work item to show details for",
        ))
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        Some(vec!["details", "detail"])
    }

    fn name(&self) -> &str {
        "show"
    }
}

/// Execute the show command.
fn execute(args: &Vec<arg::Value>, _options: &HashMap<&str, option::Value>) {
    let id = args[0].int().unwrap();

    match persistence::find_item_by_id(id) {
        Ok(optional_item) => match optional_item {
            Some(item) => print_item(item),
            None => println!("Could not find work item with ID {}.", id),
        },
        Err(e) => println!(
            "Failed to show details for work item with ID {}. Error: '{}'.",
            id, e
        ),
    }
}

/// Print the work item.
fn print_item(item: WorkItem) {
    print_header(&format!(
        "Details for work item with ID {}",
        item.id().unwrap()
    ));
    println!();

    println!("{}", "# Description".underlined());

    println!("{}", item.description());

    println!();

    println!("{}", "# Status".underlined());

    println!(
        "{}",
        format!("{}", item.status()).color(colorful::Color::OrangeRed1)
    );

    println!();

    println!("{}", "# Tags".underlined());

    for tag in item.tags() {
        println!("  • {}", tag.color(colorful::Color::DeepPink2));
    }

    println!();

    println!("{}", "# Events".underlined());

    for event in item.events() {
        println!(
            "  • [{}] at {}",
            format!("{}", event.event_type()).color(colorful::Color::DarkSlateGray1),
            shared::time::get_local_date_time(event.timestamp()).format("%H:%M:%S - %A, %Y-%m-%d")
        );
    }

    println!();

    println!("{}", "# Statistics".underlined());

    println!(
        "  • Total time in progress: {}",
        shared::time::format_duration((item.time_taken() / 1000) as u32)
    );

    println!();
}

/// Print a header string to the console.
fn print_header(str: &str) {
    println!(" {} ", "-".repeat(str.len() + 2));
    println!("| {} |", str);
    println!(" {} ", "-".repeat(str.len() + 2));
}
