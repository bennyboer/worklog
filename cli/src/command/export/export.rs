use crate::command::command::Command;
use cmd_args::{arg, option, Group};
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
        export_to_markdown(options.get("path").unwrap().str().unwrap());
    } else {
        panic!("Export type '{}' currently not supported", export_type);
    }
}

fn export_to_markdown(file_path: &str) {
    let mut data = String::new();

    let entries = persistence::list_items().unwrap();
    for entry in &entries {
        data.push_str(&format!(
            "- {} ({}), took {} minutes\n",
            entry.description(),
            entry.tags().join(", "),
            entry.time_taken() / 1000 / 60
        ));
    }

    fs::write(file_path, data).expect("Unable to write export file");
}
