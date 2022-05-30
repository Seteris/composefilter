use crossterm::style::Stylize;

static ERR_INVALID_ARGS: &str = "Invalid arguments. Specify ./binary [--mute|-m] -- <command>";

pub struct ParsedArgs {
    pub mute: bool,
    pub command: Vec<String>,
}

pub fn read_args(input_args: Vec<String>) -> ParsedArgs {
    let mut args = input_args;
    args.remove(0);
    let mut parsed_args = ParsedArgs {
        mute: false,
        command: Vec::new()
    };
    println!("{:?} {}", args, args.len());
    match args.len() {
        0 | 1 => {
            println!("{} 1", ERR_INVALID_ARGS.red());
            panic!("{} 1", ERR_INVALID_ARGS);
        },
        _ => {
            if &args[0] == "--mute" || &args[1] == "-m" {
                parsed_args.mute = true;
                &args.remove(0);
            }
            if &args[0] != "--" {
                println!("{} 2", ERR_INVALID_ARGS.red());
                panic!("{} 2", ERR_INVALID_ARGS);
            }
            &args.remove(0);
            parsed_args.command = args;
            if parsed_args.command.len() == 0 {
                println!("{} 3", ERR_INVALID_ARGS.red());
                panic!("{} 3", ERR_INVALID_ARGS);
            }
        }
    }
    parsed_args
}
