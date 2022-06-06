mod argreader;
mod crossterm_execution;
mod string_builder;

use crate::crossterm_execution::crossterm_execute;
use crossterm::cursor::{MoveTo, RestorePosition, SavePosition};
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
use regex::{Regex};
use std::io::{stdout, BufRead, Write, BufReader};
use std::{env, io};
use duct::cmd;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let parsed_args = argreader::read_args(args);

    let step_regex = Regex::new(r"Step (\d+)/(\d+)").unwrap();
    let build_regex = Regex::new(r"Building (.*)").unwrap();

    let mut step_string: String = String::from("");
    let mut build_string: String = String::from("");
    let mut build_count = 0;

    crossterm_execute(Clear(ClearType::All));
    crossterm_execute(MoveTo(0, 0));

    let command = cmd!(parsed_args.command[0].clone());
    let reader = command.stderr_to_stdout().reader()?;
    let lines = BufReader::new(reader).lines();

    for line in lines {
        if step_regex.is_match(line.as_ref().unwrap()) {
            let line_clone = line.as_ref().unwrap().clone();
            let matches = step_regex.captures(line_clone.as_str()).unwrap();
            step_string = string_builder::get_info_string(matches);
        } else if build_regex.is_match(line.as_ref().unwrap()) {
            let line_clone = line.as_ref().unwrap().clone();
            let matches = build_regex.captures(line_clone.as_str()).unwrap();
            build_string = string_builder::get_build_string(matches, &mut build_count);
        }
        match parsed_args.mute {
            true => {
                crossterm_execute(SavePosition);
                print!("{}{}\n", build_string.clone().green(), step_string.clone().green());
                crossterm_execute(RestorePosition);
            }
            false => {
                println!("{}{}{}", build_string.clone().green(), step_string.clone().green(), line.unwrap());
            }
        }
        stdout().flush().unwrap();
    }
    print!("\n");
    Ok(())
}
