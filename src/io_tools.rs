use std::io::{self, Write};

pub fn read_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("There was a problem reading the command.");

    input.trim().to_string()
}

#[allow(dead_code)]
pub fn read_usize() -> usize {
    read_line()
        .trim()
        .parse::<usize>()
        .expect("There was a problem parsing the index to usize.")
}

pub fn print_and_flush(string: &str) {
    print!("{string}");

    io::stdout()
        .flush()
        .expect("There was a problem doing flush.");
}
