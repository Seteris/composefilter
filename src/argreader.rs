use crossterm::style::Stylize;

static ERR_INVALID_ARGS: &str = "Invalid arguments. Specify ./binary [--mute|-m] -- <command>";

pub struct ParsedArgs {
    pub mute: bool,
    pub command: Vec<String>,
}

pub fn read_args(input_args: Vec<String>) -> ParsedArgs {
    let mut args = input_args;
    let _ = args.remove(0);
    let mut parsed_args = ParsedArgs {
        mute: false,
        command: Vec::new()
    };
    match args.len() {
        0 | 1 => {
            println!("{}", ERR_INVALID_ARGS.red());
            panic!("{}", ERR_INVALID_ARGS);
        },
        _ => {
            if &args[0] == "--mute" || &args[1] == "-m" {
                parsed_args.mute = true;
                let _ = &args.remove(0);
            }
            if &args[0] != "--" {
                println!("{}", ERR_INVALID_ARGS.red());
                panic!("{}", ERR_INVALID_ARGS);
            }
           let _ = &args.remove(0);
            parsed_args.command = args;
            if parsed_args.command.len() == 0 {
                println!("{}", ERR_INVALID_ARGS.red());
                panic!("{}", ERR_INVALID_ARGS);
            }
        }
    }
    parsed_args
}
