#![allow(warnings)]

use std::fs::File;
use std::io::Read;

pub fn runner() -> (i32, i32) {
    //NOTE: Execute cargo run from project root (not /src/), otherwise
    //a file not found error may occur.
    // Reads input from file
    let mut input = String::new();
    match File::open("./config/aoc1503_1.txt") {
        Ok(mut file) => match file.read_to_string(&mut input) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error reading file: {}", err);
            }
        },
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }
    // Pushes each input char to a vector for processing
    let mut string_as_vec: Vec<char> = Vec::new();
    for char in input.chars() {
        string_as_vec.push(char)
    }

    // Part one
    let mut results = (0, 0);

    // Part two

    results
}

fn core_logic() -> i32 {
    23
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /** Tests the package calculator */
    fn d05_1() {}
}
