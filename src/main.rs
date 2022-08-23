use std::io::{self, Write};

fn main() {
    println!("Welcome to your TODO app");
    println!("If you need help or if you don't remember the commands type help");

    loop {
        print!(">>> ");

        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin().read_line(&mut command).unwrap();

        command = command.trim().to_string();

        match &command[..] {
            "delete" => delete(),
            "exit" => exit(),
            "help" => help(),
            "list" => list(),
            "new" => new(),
            "update" => update(),
            _ => invalid_command(&command[..]),
        };
    }
}

fn delete() {}

fn exit() {}

fn help() {}

fn list() {}

fn new() {}

fn update() {}

fn invalid_command(command: &str) {
    println!("The command {command} is invalid.");
}
