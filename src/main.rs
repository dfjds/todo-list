use clap::{self, Arg, ArgAction, Command};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    description: String,
    completed: bool,
}

// helper functions
fn load_to_list(path: &str) -> Vec<TodoItem> {
    // Attempt to open the file; if it doesn't exist then create it with an empty array
    let content = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
}
fn save_todo_list(path: &str, list: &Vec<TodoItem>) {
    let json = serde_json::to_string_pretty(list).expect("Failed to serialize todo list");
    fs::write(path, json).expect("Failed to write to file");
}

fn clear(path: &str) {
    println!("Cleared the todo list.");
    save_todo_list(path, &Vec::new());
}

fn main() {
    let matches = Command::new("todo")  // creating a CLI command
        .version("1.0")  // Metadata
        .author("dsfg")  // Metadata
        .about("A simple CLI based Todo List written in rust")  // Metadata
        .arg(  // Creating a flag
            Arg::new("add")  // new argument
                .short('a')  // flag argument e.g. : 'todo -a [item]'
                .long("add")  // alternative flag item e.g. : 'todo --add [item]'
                .value_name("ITEM")  // input requremnet usually ment for help and error messages
                .help("Add a new todo item.\nFor items with morw than one word add \"\" ")  // help message
                .num_args(1..)  // allows for multiple inputs
                .action(ArgAction::Append)
                .value_parser(clap::value_parser!(String))  // Define value type to input
        )
        .arg(  // Creating a flag
            Arg::new("remove")  // new argument
                .short('r')  // flag argument e.g. : 'todo -r [item]'
                .long("remove")  // alternative flag item e.g. : 'todo --remove [item]'
                .value_name("ITEM")  // input requremnet usually ment for help and error messages
                .help("Remove a todo item.\nFor items with more than one word add \"\" ")  // help message
                .num_args(1..)  // allows for multiple inputs
                .action(ArgAction::Append)
                .value_parser(clap::value_parser!(String))  // Define value type to input
        )
        .arg(
            Arg::new("complete")  // new argument
                .short('c')  // flag argument e.g. : 'todo -c [item]'
                .long("complete")  // alternative flag item e.g. : 'todo --complete [item]'
                .value_name("ITEM")  // input requremnet usually ment for help and error messages
                .help("Mark a todo item as complete.\nFor items with more than one word add \"\" ")  // help message
                .num_args(1..)  // allows for multiple inputs
                .action(ArgAction::Append)
                .value_parser(clap::value_parser!(String))  // Define value type to input
        )
        .arg(
            Arg::new("uncomplete")  // new argument
                .short('u')  // flag argument e.g. : 'todo -u [item]'
                .long("uncomplete")  // alternative flag item e.g. : 'todo --uncomplete [item]'
                .value_name("ITEM")  // input requremnet usually ment for help and error messages
                .help("Unmark a todo item as complete.\nFor items with more than one word add \"\" ")  // help message
                .num_args(1..)  // allows for multiple inputs
                .action(ArgAction::Append)
                .value_parser(clap::value_parser!(String))  // Define value type to input
        )
        .arg(
            Arg::new("clear")  // new argument
                .short('x') // flag
                .long("clear")
                .help("Delete all items in the todo list")  // help message
                .action(ArgAction::SetTrue),  // no input needed
        )
        .arg(
            Arg::new("list")  // new argument
                .short('l')  // flag argument e.g. : 'todo -l'
                .long("list")  // alternative flag item e.g. : 'todo --list'
                .help("List all todo items")  // help message
                .action(ArgAction::SetTrue),  // this flag doesn't require input
        )
        .get_matches();

    //  Defining path to the save file
    let todo_list_path = "todo.json";

    if matches.get_flag("clear") {
        clear(todo_list_path);
        return;
    }

    let mut todo_list = load_to_list(todo_list_path);

    // If the "add" argument is provided, add each item to the todo list.
    if let Some(items) = matches.get_many::<String>("add") {
        for item in items {
            add(item.to_string(), &mut todo_list);
        }
        // Save the updated list to disk.
        save_todo_list(todo_list_path, &todo_list);
    }
    // If the "add" argument is provided, add each item to the todo list.
    else if let Some(items) = matches.get_many::<String>("remove") {
        for item in items {
            remove(item.to_string(), &mut todo_list);
        }
        // Save the updated list to disk.
        save_todo_list(todo_list_path, &todo_list);
    }
    // If the "add" argument is provided, add each item to the todo list.
    else if let Some(items) = matches.get_many::<String>("complete") {
        for item in items {
            complete(item.to_string(), &mut todo_list);
        }
        // Save the updated list to disk.
        save_todo_list(todo_list_path, &todo_list);
    }
    // If the "add" argument is provided, add each item to the todo list.
    else if let Some(items) = matches.get_many::<String>("uncomplete") {
        for item in items {
            uncomplete(item.to_string(), &mut todo_list);
        }
        // Save the updated list to disk.
        save_todo_list(todo_list_path, &todo_list);
    }

    // handle arguments
    if let Some(item) = matches.get_one::<String>("add") {
        add(item.to_string(), &mut todo_list);
        
        // save the todo list after eddit
        save_todo_list(todo_list_path, &todo_list);
    }
    else if let Some(item) = matches.get_one::<String>("remove") {
        remove(item.to_string(), &mut todo_list);

        // save the todo list after eddit
        save_todo_list(todo_list_path, &todo_list);
    }
    else if let Some(item) = matches.get_one::<String>("complete") {
        complete(item.to_string(), &mut todo_list);

        // save the todo list after eddit
        save_todo_list(todo_list_path, &todo_list);
    }
    else if let Some(item) = matches.get_one::<String>("uncomplete") {
        uncomplete(item.to_string(), &mut todo_list);

        // save the todo list after eddit
        save_todo_list(todo_list_path, &todo_list);
    }
    else if matches.get_flag("list") {
        list(&todo_list);
    }

}

fn add(item: String, list: &mut Vec<TodoItem>) {
    println!("Adding: {}", item);

    list.push(TodoItem {
        description: item,
        completed: false
    });
}
fn remove(item: String, list: &mut Vec<TodoItem>) {
    println!("Removing: {}", item);
    list.retain(|todo| todo.description != item);
}

fn complete(item: String, list: &mut Vec<TodoItem>) {
    println!("Marked: {} as Complete", item);
    for todo in list.iter_mut() {
        if todo.description == item {
            todo.completed = true;
        }
    }
}

fn uncomplete(item: String, list: &mut Vec<TodoItem>) {
    println!("Unmarking: {} as Complete", item);
    for todo in list.iter_mut() {
        if todo.description == item {
            todo.completed = false;
        }
    }
}
fn list(list: &Vec<TodoItem>) {
    println!("Todo list: ");
    for todo in list.iter() {
        println!(
            "[{}] {}",
            if todo.completed { "x" } else { " " },
            todo.description
        );
    }
}