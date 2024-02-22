#![allow(warnings)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

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
    results.0 = unique(location_history(&string_as_vec));
    // Part two
    let routes: (Vec<char>, Vec<char>) = path_splitter(&string_as_vec);
    let combined = path_concatenator(routes);
    results.1 = unique(combined);
    results
}

// Filters the history vector for unique instances
fn unique(v: Vec<(i32, i32)>) -> i32 {
    let mut unique = HashMap::new();
    for location in &v {
        *unique.entry(location).or_insert(0) += 1;
    }
    unique.len() as i32
}

fn location_history(v: &Vec<char>) -> Vec<(i32, i32)> {
    // Prints a matrix
    //for i in &matrix {
    //    for e in i {
    //        print!("{:?} ", e)
    //    }
    //    println!();
    //}

    // Takes the directions vector and creates a location point
    // represented by a tuple which is recorded in a history vector
    let mut history: Vec<(i32, i32)> = Vec::new();
    let mut location = (0, 0);
    history.push(location); // Gotta start somewhere!
    let mut char_index = 0;
    while char_index < v.len() {
        if v[char_index] == '>' {
            location.0 += 1;
        }
        if v[char_index] == '<' {
            location.0 -= 1;
        }
        if v[char_index] == '^' {
            location.1 += 1;
        }
        if v[char_index] == 'v' {
            location.1 -= 1;
        }
        history.push(location);
        char_index += 1;
    }
    history
}

fn path_splitter(v: &Vec<char>) -> (Vec<char>, Vec<char>) {
    //let paths: HashMap<Vec<i32>, Vec<i32>> = HashMap::new();
    let mut paths: (Vec<char>, Vec<char>) = (Vec::new(), Vec::new());
    let mut robo: Vec<char> = Vec::new();
    for (i, v) in v.iter().enumerate() {
        if i % 2 == 0 {
            paths.0.push(*v);
        } else {
            paths.1.push(*v);
        }
    }
    paths
}

fn path_concatenator(routes: (Vec<char>, Vec<char>)) -> Vec<(i32, i32)> {
    let mut combined: Vec<(i32, i32)> = Vec::new();
    let santa: Vec<(i32, i32)> = location_history(&routes.0);
    let robo: Vec<(i32, i32)> = location_history(&routes.1);
    for x in santa {
        combined.push(x);
    }
    for x in robo {
        combined.push(x);
    }
    combined
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /** Tests the package calculator */
    fn first() {
        let v: Vec<char> = vec!['>'];
        let vals = unique(location_history(&v));
        assert_eq!(vals, 2);

        let v: Vec<char> = vec!['^', '>', 'v', '<'];
        let vals = unique(location_history(&v));
        assert_eq!(vals, 4);

        let v: Vec<char> = vec!['^', 'v', '^', 'v', '^', 'v', '^', 'v', '^', 'v'];
        let vals = unique(location_history(&v));
        assert_eq!(vals, 2);
    }
    #[test]
    /** Tests the package calculator */
    fn second() {
        let v: Vec<char> = vec!['^', 'v'];
        let routes: (Vec<char>, Vec<char>) = path_splitter(&v);
        let combined = path_concatenator(routes);
        assert_eq!(unique(combined), 3);

        let v: Vec<char> = vec!['^', '>', 'v', '<'];
        let routes: (Vec<char>, Vec<char>) = path_splitter(&v);
        let combined = path_concatenator(routes);
        assert_eq!(unique(combined), 3);

        let v: Vec<char> = vec!['^', 'v', '^', 'v', '^', 'v', '^', 'v', '^', 'v'];
        let routes: (Vec<char>, Vec<char>) = path_splitter(&v);
        let combined = path_concatenator(routes);
        assert_eq!(unique(combined), 11);
    }
}
