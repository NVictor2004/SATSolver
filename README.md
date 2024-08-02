# SAT Solver
## Functionality
This is a simple SAT Solver library that provides functionality to determine every satisfying interpretation of a Boolean formula.

Currently, the formula must be written in a file, which is provided as input.

## Supported Operations
Currently, the following Boolean operations are supported:
- AND, written with a `'&'`
- OR, written with a `'|'`
- NOT, written with a `'!'`

Brackets can also be written, using the `()` characters.

## Setup Requirements
Compilation of this program requires Rustup. A description of how to download this can be found at `doc.rust-lang.org/book/`. 

## Compilation and Execution
The file `main.rs` contains an example usage of the library. To compile and run this, use the command `cargo run -- filename`, where `filename` is the name of the file containing your Boolean formula.

## Example
Given the input `!(a & b | !c) & !(b | !c)` in the file `formula.txt`, after running `cargo run -- formula.txt`, the two correct solutions `c & !b & a` and `c & !b & !a` are outputted to STDOUT. 
