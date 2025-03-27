use std::env;

mod commands;
use commands::help;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help::execute();
    } else {
        let command = &args[1];
        match command.as_str() {
            //"build" | "b" => build::execute(),
            //"check" | "c" => check::execute(),
            //"flash" | "f" | "d" => flash::execute(),
            //"run" | "r" => run::execute(),
            "help" | "h" => help::execute(),
            //"summary" | "s" => summary::execute(),
            _ => {
                println!("Unknown command: {}, Try clay help", command);
            }
        }
    }
}