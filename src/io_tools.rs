use std::io::{self, Write};

pub fn read_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("There was a problem reading the command.");

    input.trim().to_string()
}
