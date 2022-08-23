use std::io::{self, Write};

fn main() {
    println!("Welcome to your TODO app");
    println!("If you need help or if you don't remember the commands type help");

    loop {
        print!(">>> ");

        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin().read_line(&mut command).unwrap();

        match &command[..] {
            "exit" => exit(),
            "delete" => delete(),
            "list" => list(),
            "new" => new(),
            "update" => update(),
            _ => invalid_command(),
        };
    }
}

fn new() {}

fn exit() {}

fn list() {}

fn update() {}

fn delete() {}

fn invalid_command() {}
