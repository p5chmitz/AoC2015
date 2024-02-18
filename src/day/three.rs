use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /** Tests the package calculator */ 
    fn d3_1_2() {
        //let v = vec![1, 1, 10];
        //let p = Package::build(&v);
        //let result = Package::calculate_area(&p);
        //assert_eq!(result, 43);
    }
}

pub fn runner() -> (i32, i32) {
    //NOTE: Execute cargo run from project root (not /src/), otherwise
    //a file not found error may occur.
    let mut input = String::new();
    match File::open("./config/aoc1503_1.txt") {
        Ok(mut file) => {
            match file.read_to_string(&mut input) {
                Ok(_) => {}
                Err(err) => {eprintln!("Error reading file: {}", err);}
            }
        }
        Err(err) => {eprintln!("Error opening file: {}", err);}
    }
    let test_data = String::from(">^^v^");
    //let total = core_logic(test_data);
    let total = core_logic(input);
    total
}

pub fn core_logic(s: String) -> (i32, i32) {
    // Pushes each input char to a vector,
    let mut string_as_vec: Vec<char> = Vec::new();
    for char in s.chars() {
        string_as_vec.push(char)
    };

    // Prints a matrix
    //for i in &matrix {
    //    for e in i {
    //        print!("{:?} ", e)
    //    }
    //    println!();
    //}
    //println!("=>");

    // Santa's path
    let mut location = (0, 0);
    let mut history: Vec<(i32, i32)> = Vec::new();
    let mut char_index = 0;
    while char_index < string_as_vec.len() {
        if string_as_vec[char_index] == '>' {
            location.0 += 1;
        }
        if string_as_vec[char_index] == '<' {
            location.0 -= 1;
        }
        if string_as_vec[char_index] == '^' {
            location.1 += 1;
        }
        if string_as_vec[char_index] == 'v' {
            location.1 -= 1;
        }
        history.push(location);
        char_index += 2;
    }

    // Robo-Santa's path
    let mut location = (0, 0);
    let mut history: Vec<(i32, i32)> = Vec::new();
    let mut char_index = 1;
    while char_index < string_as_vec.len() {
        if string_as_vec[char_index] == '>' {
            location.0 += 1;
        }
        if string_as_vec[char_index] == '<' {
            location.0 -= 1;
        }
        if string_as_vec[char_index] == '^' {
            location.1 += 1;
        }
        if string_as_vec[char_index] == 'v' {
            location.1 -= 1;
        }
        history.push(location);
        char_index += 2;
    }


    let mut unique = HashMap::new();
    for location in &history {
        *unique.entry(location).or_insert(0) += 1;
    }
    println!("{:?}", unique);

    ((unique.len() as i32 + 1), 0)
}

