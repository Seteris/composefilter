mod argreader;
mod crossterm_execution;

use crate::crossterm_execution::crossterm_execute;
use crossterm::cursor::{MoveTo, RestorePosition, SavePosition};
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
use regex::Regex;
use std::io::{stdout, BufRead, Write};
use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mute = argreader::read_args(args);
    let stdin = io::stdin();
    let search = Regex::new(r"([Hit|Get]:(\d+))").unwrap();
    crossterm_execute(Clear(ClearType::All));
    crossterm_execute(MoveTo(0, 0));
    for line in stdin.lock().lines() {
        crossterm_execute(SavePosition);
        if search.is_match(line.as_ref().unwrap()) {
            crossterm_execute(MoveTo(0, 0));
            let line_clone = line.as_ref().unwrap().clone();
            for cap in search.captures_iter(line_clone.as_str()) {
                print!("Fetched {} update packages", &cap[2]);
            }
            crossterm_execute(RestorePosition);
        }
        if !mute {
            println!("{}", line.unwrap());
        }
        stdout().flush().unwrap();
    }
    print!("\n");
    Ok(())
}
