use std::io::{self, Write};

mod commands;
mod io_tools;

fn main() {
    println!("Welcome to your TODO app.");
    println!("If you need help or if you don't remember the commands type help.");

    let mut tasks = Vec::new();

    loop {
        print!(">>> ");

        io::stdout()
            .flush()
            .expect("There was a problem doing flush.");

        let command = io_tools::read_line();

        commands::execute_command(&mut tasks, command);
    }
}
