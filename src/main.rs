use std::io;
use std::io::Write;

mod shell;

use crate::shell::{Shell, ShellCommand};

/// Shell command for sending a message to mission control.
struct SendMessageCommand {}

impl ShellCommand for SendMessageCommand {
    fn execute(&self) -> bool {
        println!("Enter message:");
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // TODO: actually send message somewhere
        println!("Message sent");
        false
    }
    fn describe(&self) -> String {
        String::from("Send a message to mission control")
    }
}

/// Run Curiosity OS.
fn main() {
    let mut shell = Shell::new("Welcome to Curiosity", "Goodbye");
    shell.add_command("msg", Box::new(SendMessageCommand {}));
    shell.run();
}
