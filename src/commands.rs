//! Shell commands.

use std::io;
use std::io::Write;

/// A command that can be executed in the shell.
pub trait ShellCommand {
    /// Execute the command. A return value of true will cause the shell to
    /// exit.
    fn execute(&self) -> bool;

    /// Describe the command. Can be used to generate help text.
    fn describe(&self) -> String;
}

/// A command that exits the shell.
pub struct ExitCommand {
    /// A message that will be printed just before exiting the shell.
    pub exit_string: String,
}

impl ShellCommand for ExitCommand {
    fn execute(&self) -> bool {
        println!("{}", self.exit_string);
        true
    }
    fn describe(&self) -> String {
        String::from("Exit the command interpreter")
    }
}

/// Shell command for sending a message to mission control.
pub struct SendMessageCommand {}

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

/// Shell command for checking status of ship systems
pub struct CheckStatusCommand {}

impl ShellCommand for CheckStatusCommand {
    fn execute(&self) -> bool {
        println!("Engines:      Nominal");
        println!("Shields:      Nominal");
        println!("Life support: Nominal");
        false
    }
    fn describe(&self) -> String {
        String::from("Check systems status")
    }
}