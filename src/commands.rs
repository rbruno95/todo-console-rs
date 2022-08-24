use std::io::{self, Write};
use std::process;

#[path = "io_tools.rs"]
mod io_tools;

pub struct Task {
    pub name: String,
    pub done: bool,
}

pub fn execute_command(tasks: &mut Vec<Task>, command: String) {
    match &command[..] {
        "delete" => delete(tasks),
        "exit" => exit(),
        "help" => help(),
        "list" => list(tasks),
        "new" => new(tasks),
        "update" => update(tasks),
        _ => invalid_command(&command[..]),
    };
}

fn delete(tasks: &mut Vec<Task>) {
    if tasks.len() == 0 {
        println!("You don't have tasks.");
        return;
    }

    for (i, task) in tasks.iter().enumerate() {
        println!("{i} -> {}", task.name);
    }

    print!("Enter the number of the task: ");

    io::stdout()
        .flush()
        .expect("There was a problem doing flush.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("There was a problem reading the index.");

    let index = index
        .trim()
        .parse::<usize>()
        .expect("There was a problem parsing the index to usize.");

    if tasks.len() > index {
        tasks.remove(index);
    } else {
        println!("Select one of the listed numbers.");
    }
}

fn exit() {
    process::exit(0);
}

fn help() {
    println!("delete: interface to delete a task");
    println!("exit: exit the program");
    println!("help: show the available commands");
    println!("list: show all the tasks");
    println!("new: interface to create a task");
    println!("update: interface to edit a task");
}

fn list(tasks: &mut Vec<Task>) {
    println!("TODO:");

    for task in tasks.iter().filter(|task| !task.done) {
        println!("[ ] {}", task.name);
    }

    println!("DONE:");

    for task in tasks.iter().filter(|task| task.done) {
        println!("[X] {}", task.name);
    }
}

fn new(tasks: &mut Vec<Task>) {
    print!("Enter task name: ");

    io::stdout()
        .flush()
        .expect("There was a problem doing flush.");

    let name = io_tools::read_line();

    tasks.push(Task { name, done: false });
}

fn update(tasks: &mut Vec<Task>) {
    if tasks.len() == 0 {
        println!("You don't have tasks.");
        return;
    }

    for (i, task) in tasks.iter().enumerate() {
        println!("{i} -> {}", task.name);
    }

    print!("Enter the number of the task: ");

    io::stdout()
        .flush()
        .expect("There was a problem doing flush.");

    let index = io_tools::read_line()
        .trim()
        .parse::<usize>()
        .expect("There was a problem parsing.");

    if tasks.len() > index {
        tasks[index].done = !tasks[index].done;
    } else {
        println!("Select one of the listed numbers.");
    }
}

fn invalid_command(command: &str) {
    println!("The command {command} is invalid.");
}
