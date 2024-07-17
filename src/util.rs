
use std::env::Args;

// Get the filename, if provided
pub fn get_filename(mut args: Args) -> Result<String, &'static str> {

    // Skip Executable Name
    args.next();

    // Return filename if provided
    // Or an error message if not
    match args.next() {
        Some(filename) => Ok(filename),
        None => Err("Please provide a filename!"),
    }
}