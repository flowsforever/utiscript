mod commands;
use commands::{help/*, build, check, flash, run, summary*/};

fn main() {
    let command = "help";

    match command {
        //"build" | "b" => build::execute(),
        //"check" | "c" => check::execute(),
        //"flash" | "f" | "d" => flash::execute(),
        //"run" | "r" => run::execute(),
        "help" | "h" => help::execute(),
        //"summary" | "s" => summary::execute(),
        _ => println!("Unknown command!"),
    }
}
