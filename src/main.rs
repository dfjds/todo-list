// use std::fs;
use clap::{self, Arg, ArgAction, Command};
// use anyhow::{Context, Result};

fn main() {
    let matches = Command::new("todo")  // creating a CLI command
        .version("1.0")  // Metadata
        .author("dsfg")  // Metadata
        .about("A simple CLI based Todo List written in rust")  // Metadata
        // Creating a flag
        .arg(
            Arg::new("add")  // new argument
                .short('a')  // flag argument e.g. : 'todo -a [item]'
                .long("add")  // alternative flag item e.g. : 'todo --add [item]'
                .value_name("ITEM")  // input requremnet usually ment for help and error messages
                .help("Add a new todo item")  // help message
                .value_parser(clap::value_parser!(String))  // Define value type to input
        )
        // Creating a flag
        .arg(
            Arg::new("remove")  // new argument
                .short('r')  // flag argument e.g. : 'todo -r [item]'
                .long("remove")  // alternative flag item e.g. : 'todo --remove [item]'
                .value_name("ITEM")  // input requremnet usually ment for help and error messages
                .help("Remove a todo item")  // help message
                .value_parser(clap::value_parser!(String))  // Define value type to input
        )
        .arg(
            Arg::new("complete")  // new argument
                .short('c')  // flag argument e.g. : 'todo -c [item]'
                .long("complete")  // alternative flag item e.g. : 'todo --complete [item]'
                .value_name("ITEM")  // input requremnet usually ment for help and error messages
                .help("Mark a todo item as complete")  // help message
                .value_parser(clap::value_parser!(String))  // Define value type to input
        )
        .arg(
            Arg::new("uncomplete")  // new argument
                .short('u')  // flag argument e.g. : 'todo -u [item]'
                .long("uncomplete")  // alternative flag item e.g. : 'todo --uncomplete [item]'
                .value_name("ITEM")  // input requremnet usually ment for help and error messages
                .help("Unmark a todo item as complete")  // help message
                .value_parser(clap::value_parser!(String))  // Define value type to input
        )
        // Creating a flag
        .arg(
            Arg::new("list")  // new argument
                .short('l')  // flag argument e.g. : 'todo -l'
                .long("list")  // alternative flag item e.g. : 'todo --list'
                .help("List all todo items")  // help message
                .action(ArgAction::SetTrue),  // this flag doesn't require input
        )
        .get_matches();

    // handle arguments
    if let Some(item) = matches.get_one::<String>("add") {
        println!("Adding: {}", item);
    }
}
