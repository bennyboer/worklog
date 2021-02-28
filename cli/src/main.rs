use cmd_args::{parser, Group};
use std::process;

mod command;

fn main() {
    let mut group = Group::new(
        Box::new(|_args, _options| {
            println!("### Incorrect usage ###");
            println!("Pass '--help' to see all available options.");
            process::exit(1);
        }),
        "Tool to log your work",
    );

    // Add all sub-commands
    for command in &command::COMMANDS {
        group = group.add_child(command.name(), command.aliases(), command.build());
    }

    // Start the command line parser
    parser::parse(group, None).unwrap();
}
