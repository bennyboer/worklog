use crate::command::command::Command;
use cmd_args::{arg, option, Group};
use persistence::work_item::WorkItem;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::iter::FromIterator;

/// Command used to export work items.
pub struct EditCommand {}

impl Command for EditCommand {
    fn build(&self) -> Group {
        Group::new(
            Box::new(|args, options| execute(args, options)),
            "Edit a work item",
        )
        .add_argument(arg::Descriptor::new(
            arg::Type::Int,
            "ID of the work item to edit",
        ))
        .add_option(option::Descriptor::new(
            "description",
            option::Type::Str {
                default: String::from(""),
            },
            "New description of the work item",
        ))
        .add_option(option::Descriptor::new(
            "tags",
            option::Type::Str {
                default: String::from(""),
            },
            "New tags for the work item",
        ))
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        Some(vec!["e", "update"])
    }

    fn name(&self) -> &str {
        "edit"
    }
}

/// Execute the edit command.
fn execute(args: &Vec<arg::Value>, options: &HashMap<&str, option::Value>) {
    let id = args[0]
        .int()
        .expect("Expected to have an ID supplied as first argument");

    let description = options
        .get("description")
        .unwrap()
        .str()
        .map(|v| {
            if v.is_empty() {
                None
            } else {
                Some(v.to_owned())
            }
        })
        .unwrap();

    let tags: Option<Vec<String>> = options
        .get("tags")
        .unwrap()
        .str()
        .map(|v| {
            if v.is_empty() {
                None
            } else {
                Some(v.split(",").map(|s| s.trim().to_owned()).collect())
            }
        })
        .unwrap();

    match persistence::find_item_by_id(id) {
        Ok(item) => match item {
            Some(mut item) => match update_work_item(&mut item, description, tags) {
                Ok(_) => println!("Updated work item with ID {}.", id),
                Err(e) => println!("Could not edit work item with ID {}. Error: '{}'.", id, e),
            },
            None => println!("Could not find work item with ID {}.", id),
        },
        Err(e) => println!(
            "An error occurred while trying to access the work item with ID {}. Error: '{}'",
            id, e
        ),
    };
}

/// Update the passed work item with the given optional changes.
fn update_work_item(
    item: &mut WorkItem,
    description: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    if description.is_some() {
        item.set_description(description.unwrap());
    }

    if tags.is_some() {
        item.set_tags(HashSet::from_iter(tags.unwrap().into_iter()));
    }

    // Persist changes
    persistence::update_items(vec![item])
}
