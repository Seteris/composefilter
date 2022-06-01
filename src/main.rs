mod argreader;
mod crossterm_execution;

use crate::crossterm_execution::crossterm_execute;
use crossterm::cursor::{MoveTo, RestorePosition, SavePosition};
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
use regex::{Captures, Regex};
use std::io::{stdout, BufRead, Write, BufReader};
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

fn get_build_string(matches: Captures, build_count: &mut i32) -> String {
    let mut build_string = String::from("Build ");
    *build_count += 1;
    build_string.push_str(&*build_count.to_string());
    build_string.push_str("(");
    build_string.push_str(&matches[1]);
    build_string.push_str(") ");
    build_string
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let parsed_args = argreader::read_args(args);

    let step_regex = Regex::new(r"Step (\d+)/(\d+)").unwrap();
    let build_regex = Regex::new(r"Building (.{50})").unwrap();

    let mut step_string: String = String::from("");
    let mut build_string: String = String::from("");
    let mut build_count = 0;

    crossterm_execute(Clear(ClearType::All));
    crossterm_execute(MoveTo(0, 0));

    let mut command = Command::new(parsed_args.command[0].clone())
        .args(parsed_args.command[1..].as_ref())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    match command.stdout {
        Some(ref mut out) => {
            let buf_reader = BufReader::new(out);
            for line in buf_reader.lines() {
                if step_regex.is_match(line.as_ref().unwrap()) {
                    let line_clone = line.as_ref().unwrap().clone();
                    let matches = step_regex.captures(line_clone.as_str()).unwrap();
                    step_string = get_info_string(matches);
                }
                if build_regex.is_match(line.as_ref().unwrap()) {
                    let line_clone = line.as_ref().unwrap().clone();
                    let matches = build_regex.captures(line_clone.as_str()).unwrap();
                    build_string = get_build_string(matches, &mut build_count);
                }
                match parsed_args.mute {
                    true => {
                        crossterm_execute(SavePosition);
                        print!("{}{}{}", build_string.clone().green(), step_string.clone().green(), if parsed_args.mute { "\n" } else { "" });
                        crossterm_execute(RestorePosition);
                    }
                    false => {
                        print!("{}{}{}", build_string.clone().green(), step_string.clone().green(), if parsed_args.mute { "\n" } else { "" });
                        println!("{}", line.unwrap());
                    }
                }
                stdout().flush().unwrap();
            }
        }
        None => {},
    };
    print!("\n");
    Ok(())
}
