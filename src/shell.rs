//! A generic, extensible command interpreter.

use std::collections::HashMap;
use std::io;
use std::io::Write;
use itertools::Itertools;

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

/******************
 * Shell Commands *
 ******************/

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
