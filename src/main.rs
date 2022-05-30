mod argreader;
mod crossterm_execution;

use crate::crossterm_execution::crossterm_execute;
use crossterm::cursor::{MoveTo, RestorePosition, SavePosition};
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
use regex::{Captures, Regex};
use std::io::{stdout, BufRead, Write};
use std::{env, io};
use std::process::{Command, Stdio};

fn get_info_string(matches: Captures) -> String {
    let mut info_string = String::from("Step ");
    info_string.push_str(&matches[1]);
    info_string.push_str("/");
    info_string.push_str(&matches[2]);
    info_string.push_str(" ");
    info_string
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let parsed_args = argreader::read_args(args);
    let stdin = io::stdin();
    let search = Regex::new(r"Step (\d+)/(\d+)").unwrap();
    let mut command = Command::new(parsed_args.command[0].clone())
        .args(parsed_args.command[1..].as_ref())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");
    let out = command.wait_with_output().expect("Failed to wait on child");
    println!("{:?}", out);
    panic!("WIP");
    crossterm_execute(Clear(ClearType::All));
    crossterm_execute(MoveTo(0, 0));
    let mut out_string: String = String::from("");
    for line in stdin.lock().lines() {
        if search.is_match(line.as_ref().unwrap()) {
            let line_clone = line.as_ref().unwrap().clone();
            let matches = search.captures(line_clone.as_str()).unwrap();
            out_string = get_info_string(matches);
        }
        match parsed_args.mute {
            true => {
                crossterm_execute(SavePosition);
                print!("{}{}", out_string.clone().green(), if parsed_args.mute { "\n" } else { "" });
                crossterm_execute(RestorePosition);
            }
            false => {
                print!("{}{}", out_string.clone().green(), if parsed_args.mute { "\n" } else { "" });
                println!("{}", line.unwrap());
            }
        }
        stdout().flush().unwrap();
    }
    print!("\n");
    Ok(())
}
