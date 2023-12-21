use std::fs::File;
use std::io::Read;

pub fn aoc1501_1() {
    //Should result in 4
    //let input = String::from("(()()(((()");
    let mut input = String::new();

    //Reads from file
    match File::open("./input_files/aoc1501_1.txt") {
        Ok(mut file) => {
            match file.read_to_string(&mut input) {
                Ok(_) => {
                    //println!("File contents:\n{}", input);
                }
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }

    //Parses file and prints counter
    let mut counter = 0;
    let mut p = 1;
    for character in input.chars() {
        if character == '(' {
            counter += 1;
        }
        if character == ')' {
            counter -= 1;
        }
        if counter == -1 {
            println!("The elevator goes into the basement on the {} iteration.", p);
            break
        }
        p += 1;
    }
    println!("The resultant floor is: {}", counter);
}

fn main() {
    aoc1501_1();
}
