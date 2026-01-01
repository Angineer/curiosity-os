//! A generic, extensible command interpreter.

use std::collections::HashMap;
use std::io;
use std::io::Write;
use itertools::Itertools;

use crate::commands::{ShellCommand, ExitCommand};

/// A command interpreter.
pub struct Shell {
    greeting: String,
    commands: HashMap<String, Box<dyn ShellCommand>>,
}
impl Shell {
    /// Generate a new shell with barebones commands.
    pub fn new(greeting: &str, closing: &str) -> Self {
        let mut shell = Shell {
            greeting: String::from(greeting),
            commands: HashMap::new(),
        };
        let exit_command = ExitCommand {
            exit_string: String::from(closing),
        };
        shell.add_command("exit", Box::new(exit_command));

        shell
    }

    /// Add a new command to the shell.
    pub fn add_command(&mut self, name: &str, command: Box<dyn ShellCommand>) {
        self.commands.insert(String::from(name), command);
    }

    /// Execute the shell. Will continue to accept user input until `exit` or
    /// ctrl+D is submitted.
    pub fn run(&self) {
        println!("{}", self.greeting);

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();

            // Collect input and handle EOF
            if let Ok(0) = io::stdin().read_line(&mut input) {
                println!("");
                self.commands
                    .get("exit")
                    .expect("Exit command missing")
                    .execute();
                break;
            }

            let command = input.trim();
            match command {
                "help" => self.print_help_text(),
                other => {
                    let command = self.commands.get(other);
                    match command {
                        Some(command) => {
                            if command.execute() {
                                break;
                            }
                        }
                        None => println!("Error: unknown command"),
                    }
                }
            }
        }
    }

    /// Print information about all available commands.
    fn print_help_text(&self) {
        // Find the longest command for display
        let mut width = 5;
        for (name, _) in &self.commands {
            if name.len() > width {
                width = name.len() + 1;
            }
        }

        println!("Available commands:");
        println!("{:width$} Print this help text", String::from("help"));
        for name in self.commands.keys().sorted() {
            println!("{:width$} {}", name, &self.commands[name].describe())
        }
    }
}
