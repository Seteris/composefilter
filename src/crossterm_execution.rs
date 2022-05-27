use crossterm::{Command, ExecutableCommand};
use std::io::stdout;

pub fn crossterm_execute(command: impl Command) {
    match stdout().execute(command) {
        Ok(_) => {}
        Err(error) => {
            println!("An error occurred: {}", error);
        }
    };
}
