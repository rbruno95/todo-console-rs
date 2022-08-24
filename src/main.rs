mod commands;
mod io_tools;

fn main() {
    println!("Welcome to your TODO app.");
    println!("If you need help or if you don't remember the commands type help.");

    let mut tasks = Vec::new();

    loop {
        io_tools::print_and_flush(">>> ");

        let command = io_tools::read_line();

        commands::execute_command(&mut tasks, command);
    }
}
