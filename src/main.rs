use std::io::{self, Write};
use std::process;

struct Task {
    name: String,
    done: bool,
}

fn main() {
    println!("Welcome to your TODO app");
    println!("If you need help or if you don't remember the commands type help");

    let mut tasks = Vec::new();

    loop {
        print!(">>> ");

        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin().read_line(&mut command).unwrap();

        command = command.trim().to_string();

        match &command[..] {
            "delete" => delete(&mut tasks),
            "exit" => exit(),
            "help" => help(),
            "list" => list(&mut tasks),
            "new" => new(&mut tasks),
            "update" => update(&mut tasks),
            _ => invalid_command(&command[..]),
        };
    }
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

    io::stdout().flush().unwrap();

    let mut index = String::new();

    io::stdin().read_line(&mut index).unwrap();

    let index = index.trim().to_string().parse::<usize>().unwrap();

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
    println!(
        r#"
delete: interface to delete a task
exit: exit the program
help: show the available commands
list: show all the tasks
new: interface to create a task
update: interface to edit a task
        "#
    );
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

    io::stdout().flush().unwrap();

    let mut name = String::new();

    io::stdin().read_line(&mut name).unwrap();

    name = name.trim().to_string();

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

    io::stdout().flush().unwrap();

    let mut index = String::new();

    io::stdin().read_line(&mut index).unwrap();

    let index = index.trim().to_string().parse::<usize>().unwrap();

    if tasks.len() > index {
        tasks[index].done = !tasks[index].done;
    } else {
        println!("Select one of the listed numbers.");
    }
}

fn invalid_command(command: &str) {
    println!("The command {command} is invalid.");
}
