use std::{env, io};
use std::io::{BufRead, stdout, Write};
use crossterm::{ErrorKind, ExecutableCommand};
use crossterm::cursor::{MoveTo, RestorePosition, SavePosition};
use crossterm::style::{Stylize};
use crossterm::terminal::{Clear, ClearType};
use regex::Regex;

fn print_stdout_error(error: ErrorKind) {
    println!("An error occurred: {}", error);
}

fn read_args(args: Vec<String>) -> bool {
    println!("{}", args[0]);
    match args.len() {
        1 => { false }
        2 => {
            println!("{}", args[1]);
            if &args[1] == "--mute" || &args[1] == "-m" { true } else {
                panic!("Invalid arguments. Specify ./binary [--mute|-m]");
            }
        }
        _ => {
            panic!("Invalid arguments. Specify ./binary [--mute|-m]");
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mute = read_args(args);
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
        } else if !mute {
            println!("{}", line.unwrap());
        }
        stdout.flush().unwrap();
    }
    print!("\n");
    Ok(())
}
