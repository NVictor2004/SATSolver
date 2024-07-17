
use std::{env::{self, Args}, process};
use lexer;
use parser;

fn main() {
    let filename = get_filename(env::args()).unwrap_or_else(|message| {
        eprintln!("{message}");
        process::exit(1);
    });

    let tokenstream = lexer::run(filename).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });

    println!("{tokenstream}");
    
    let expression = parser::run(tokenstream).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });

    println!("{expression}");
}

// Get the filename, if provided
fn get_filename(mut args: Args) -> Result<String, &'static str> {

    // Skip Executable Name
    args.next();

    // Return filename if provided
    // Or an error message if not
    match args.next() {
        Some(filename) => Ok(filename),
        None => Err("Please provide a filename!"),
    }
}