use std::fs::File;
use std::io::Read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /** Tests the package calculator */
    fn d2_1_1() {
        let v = vec![2, 3, 4];
        let p = Package::build(&v);
        let result = Package::calculate_area(&p);
        assert_eq!(result, 58);
    }
    #[test]
    /** Tests the package calculator */
    fn d2_1_2() {
        let v = vec![1, 1, 10];
        let p = Package::build(&v);
        let result = Package::calculate_area(&p);
        assert_eq!(result, 43);
    }
}

struct Package {
    l: i32,
    w: i32,
    h: i32,
}
impl Package {
    fn build(v: &Vec<i32>) -> Package {
        Package {
            l: v[0],
            w: v[1],
            h: v[2],
        }
    }
    fn calculate_area(&self) -> i32 {
        let a = self.l * self.w;
        let b = self.w * self.h;
        let c = self.h * self.l;
        let mut smallest = a;
        if b < smallest {
            smallest = b;
        }
        if c < smallest {
            smallest = c;
        }
        (2 * a) + (2 * b) + (2 * c) + smallest
    }
    fn calculate_length(&self) -> i32 {
        let mut v = vec![self.l, self.w, self.h];
        v.sort();
        let vol = v[0] * v[1] * v[2];
        let ribbon = (2 * v[0]) + (2 * v[1]);
        ribbon + vol
    }
}

pub fn runner() -> (i32, i32) {
    //NOTE: Execute cargo run from project root (not /src/), otherwise
    //a file not found error may occur.
    let mut input = String::new();
    match File::open("./config/aoc1502_1.txt") {
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
    let total = core_logic(input);
    total
}

/** Parses file and prints counter */
pub fn core_logic(s: String) -> (i32, i32) {
    // Pushes each input line to a vector
    let mut string_vec: Vec<&str> = Vec::new();
    for line in s.lines() {
        string_vec.push(line)
    }

    // Handles each line of the reference slice vector
    let delimiter = "x";
    let mut total_area = 0;
    let mut total_length = 0;
    for index in string_vec.iter() {
        // Creates a temporary vector to hold dimensions,
        // uses the temp vector to build a Package,
        // calculates the total surface area of the package,
        let mut temp_v = Vec::new();
        for i in index.split(delimiter) {
            let num: i32 = i.parse().unwrap();
            temp_v.push(num);
        }
        let package = Package::build(&temp_v);
        let value = Package::calculate_area(&package);
        total_area += value;

        // Calculates the total length of ribbon necessary
        let l = Package::calculate_length(&package);
        total_length += l;

        // Trace the indexes
        //println!("{:?} -> {:?} = {} && {}", &temp_v, l, value, total_area);
    }
    (total_area, total_length)
}
