use std::io;
use std::io::Write;

fn print_help_text() {
    println!("Available commands:");
    println!("help  Print this help message");
    println!("msg   Check messages or send a message");
    println!("exit  Exit the command interpreter");
}

pub fn run_shell() {
    println!("Welcome to Curiosity");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "help" => print_help_text(),
            "exit" => {
                println!("Goodbye");
                break;
            },
            &_ => println!("Error: unknown command"),
        }
    }
}
