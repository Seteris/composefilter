use crossterm::{Command, ExecutableCommand};
use std::io::stdout;
use std::process::exit;

pub fn crossterm_execute(command: impl Command) {
    match stdout().execute(command) {
        Ok(_) => {}
        Err(error) => {
            println!("An error occurred: {}", error);
            exit(-1);
        }
    };
}
