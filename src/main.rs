use std::io;
use std::io::Write;

mod shell;

use crate::shell::Shell;

fn send_message() {
    println!("Enter message:");
    print!(">> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Message sent");
}

fn main() {
    let shell = Shell::new(String::from("Welcome to Curiosity"),
                           String::from("Goodbye"));
    shell.run();
}
