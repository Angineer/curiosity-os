mod interface;
mod shell;

use crate::shell::{Shell, SendMessageCommand, CheckStatusCommand};
use crate::interface::Ui;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::io::stdout;

/// Run Curiosity OS.
// fn main() {
//     let mut shell = Shell::new("Welcome to Curiosity", "Goodbye");
//     shell.add_command("msg", Box::new(SendMessageCommand {}));
//     shell.add_command("status", Box::new(CheckStatusCommand {}));
//     shell.run();
// }

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    execute!(stdout(), EnterAlternateScreen).expect("failed to enter alternate screen");
    let run_result = Ui::default().run(&mut terminal);
    execute!(stdout(), LeaveAlternateScreen).expect("failed to leave alternate screen");
    ratatui::restore();
    run_result
}
