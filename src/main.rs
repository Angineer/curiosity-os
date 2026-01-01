mod shell;
mod commands;

use crate::commands::{SendMessageCommand, CheckStatusCommand};
use crate::shell::Shell;

/// Run Curiosity OS.
fn main() {
    let mut shell = Shell::new("Welcome to Curiosity", "Goodbye");
    shell.add_command("msg", Box::new(SendMessageCommand {}));
    shell.add_command("status", Box::new(CheckStatusCommand {}));
    shell.run();
}
