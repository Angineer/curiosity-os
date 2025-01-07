use std::collections::HashMap;
use std::io;
use std::io::Write;

pub trait ShellCommand {
    fn execute(&self) -> bool;
    fn describe(&self) -> String;
}

pub struct HelpCommand {
}
impl ShellCommand for HelpCommand {
    fn execute(&self) -> bool{
        println!("Help!");
        false
    }
    fn describe(&self) -> String{
        String::from("Available commands:")
    }
}

pub struct ExitCommand {
    exit_string: String,
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

pub struct Shell {
    opening: String,
    commands: HashMap<String, Box<dyn ShellCommand>>,
}

impl Shell {
    pub fn new(opening: String, closing: String) -> Self {
        let mut shell = Shell {
            opening,
            commands: HashMap::new(),
        };
        let help_command = HelpCommand{};
        shell.add_command("help", Box::new(help_command));
        let exit_command = ExitCommand{exit_string: closing};
        shell.add_command("exit", Box::new(exit_command));

        shell
    }

    pub fn add_command(&mut self, name: &str, command: Box<dyn ShellCommand>) {
        self.commands.insert(String::from(name), command);
    }

    pub fn run(&self) {
        println!("{}", self.opening);

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let command = self.commands.get(input.trim());
            match command {
                Some(command) => if command.execute() { break },
                None => println!("Error: unknown command"),
            }
        }
    }
}
