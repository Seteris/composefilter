use std::io;
use std::io::{BufRead, stdout, Write};
use crossterm::{ErrorKind, ExecutableCommand};
use crossterm::cursor::{MoveTo, RestorePosition, SavePosition};
use crossterm::style::{Stylize};
use crossterm::terminal::{Clear, ClearType};
use regex::Regex;

fn print_stdout_error(error: ErrorKind) {
    println!("An error occurred: {}", error);
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = stdout();
    let search = Regex::new(r"Fetched \d+,?\d* kB in \d+,?\d*s").unwrap();
    match stdout.execute(Clear(ClearType::All)) {
        Ok(_) => {}
        Err(error) => {
            print_stdout_error(error);
        }
    }
    match stdout.execute(MoveTo(0, 0)) {
        Ok(_) => {}
        Err(error) => {
            print_stdout_error(error);
        }
    }
    for line in stdin.lock().lines() {
        match stdout.execute(SavePosition) {
            Ok(_) => {}
            Err(error) => {
                print_stdout_error(error);
            }
        }
        if search.is_match(&*line.as_ref().unwrap()) {
            match stdout.execute(MoveTo(0, 0)) {
                Ok(_) => {}
                Err(error) => {
                    print_stdout_error(error);
                }
            }
            println!("{}", line.unwrap().green());
            match stdout.execute(RestorePosition) {
                Ok(_) => {}
                Err(error) => {
                    print_stdout_error(error);
                }
            }
        } else {
            println!("{}", line.unwrap());
        }
        stdout.flush().unwrap();
    }
    print!("\n");
    Ok(())
}
