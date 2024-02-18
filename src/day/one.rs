use std::fs::File;
use std::io::Read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /** Asserts that the stopping floor is 4 */ 
    fn one() {
        let input = String::from("(()()(((()");
        let tuple: (i32, i32) = (0, 4);
        assert_eq!(core_logic(input), tuple);
    }

    #[test]
    /** Asserts the interation of the first negative floor is 4 */ 
    fn two() {
        let input = String::from("(()()(((()))))())");
        let tuple: (i32, i32) = (17, -1);
        assert_eq!(core_logic(input), tuple);
    }

}

pub fn runner() -> (i32, i32) {
    //Reads input from file
    //NOTE: Execute cargo run from project root (not /src/), otherwise
    //a file not found error may occur.
    let mut input = String::new();
    match File::open("./config/aoc1501_1.txt") {
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
    let result = core_logic(input);
    result
}

/** Parses file and prints counter */
pub fn core_logic(input: String) -> (i32, i32) {
    let mut return_tuple: (i32, i32) = (0,0);
    let mut counter = 0;
    let mut counter_to_basement = 1;
    let mut basement = false;
    for character in input.chars() {
        if character == '(' {
            counter += 1;
        }
        if character == ')' {
            counter -= 1;
        }
        if basement == false {
            if counter == -1 {
                basement = true;
                return_tuple.0 = counter_to_basement;
            }
        }
        counter_to_basement += 1;
    }
    return_tuple.1 = counter;
    return return_tuple;
}

