static ERR_INVALID_ARGS: &str = "Invalid arguments. Specify ./binary [--mute|-m]";

pub fn read_args(args: Vec<String>) -> bool {
    println!("{}", args[0]);
    match args.len() {
        1 => false,
        2 => {
            println!("{}", args[1]);
            if &args[1] == "--mute" || &args[1] == "-m" {
                true
            } else {
                panic!("{}", ERR_INVALID_ARGS);
            }
        }
        _ => {
            panic!("{}", ERR_INVALID_ARGS);
        }
    }
}
