
use std::{env, process};
use satsolver::{lexer, parser, util};

fn main() {
    let filename = util::get_filename(env::args()).unwrap_or_else(|message| {
        eprintln!("{message}");
        process::exit(1);
    });

    let tokenstream = lexer::run(filename).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });
    
    let expression = parser::run(tokenstream).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });

    println!("{expression}");
}