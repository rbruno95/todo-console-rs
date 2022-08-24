use crate::commands::*;
use std::io::{self, Write};

mod commands;

fn main() {
    println!("Welcome to your TODO app.");
    println!("If you need help or if you don't remember the commands type help.");

    let mut tasks = Vec::new();

    loop {
        print!(">>> ");

        io::stdout()
            .flush()
            .expect("There was a problem doing flush.");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("There was a problem reading the command.");

        command = command.trim().to_string();

        execute_command(&mut tasks, command);
    }
}
