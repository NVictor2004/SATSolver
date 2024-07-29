
use std::{env, process};

fn main() {
    let filename = satsolver::get_filename(env::args()).unwrap_or_else(|message| {
        eprintln!("{message}");
        process::exit(1);
    });

    let tokenstream = satsolver::run_lexer(filename).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });
    
    let expression = satsolver::run_parser(tokenstream).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });

    println!("{expression}");
    
    let solutions: Vec<_> = satsolver::solve(expression).into_iter().map(| solution | solution.join(" & ")).collect();
    let solutions = solutions.join("\n");
    println!("{solutions}");
}