#![allow(warnings)]

use std::fs::File;
use std::io::Read;

pub fn runner() -> (usize, usize) {
    //NOTE: Execute cargo run from project root (not /src/), otherwise
    //a file not found error may occur.
    // Reads input from file
    let mut input = String::new();
    match File::open("./config/aoc1505_1.txt") {
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
    // Creates a Vec<Vec<char>> to process each line
    let mut lines: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut string = Vec::new();
        for char in line.chars() {
            string.push(char)
        }
        lines.push(string);
    }

    // Part one
    let part_one = part_1(&lines);

    // Part two
    let part_two = part_2(&lines);

    (part_one, part_two)
}

fn part_1(vec: &Vec<Vec<char>>) -> usize {
    // Final number of nice strings
    let mut nice = 0;

    for line in vec {
        // Each line might be nice if it gets enough points
        let mut candidate: i32 = 0;
        
        // Match against vowels "aeiou"
        let mut vowels = 0;
        for ch in line {
            match ch {
                'a' => vowels += 1,
                'e' => vowels += 1,
                'i' => vowels += 1,
                'o' => vowels += 1,
                'u' => vowels += 1,
                _ => {}
            }
        }
        if vowels >= 3 { 
            //eprintln!("line \"{:?}\" contains 3 vowels", line);
            candidate += 1 
        };

        // Match against doubles
        for (i, ch) in line.iter().enumerate() {
            if i == 0 { 
                continue 
            };
            if *ch == line[i - 1] { 
                //eprintln!("line \"{:?}\" contains a pair", line);
                candidate += 1;
                break
            }
        }

        // Match against prhibited pairs ab, cd, pq, or xy
        let prohibited = ["ab", "cd", "pq", "xy"];
        let line_str: String = line.iter().collect(); // Convert Vec<char> to String
        if prohibited.iter().any(|&pair| line_str.contains(pair)) {
            //eprintln!("line \"{:?}\" contains prohibited pair", line);
            continue
        }
        
        if candidate >= 2 { nice += 1 }
    }

    return nice;
}

use std::collections::HashMap;

/*** The string is nice if:
 - It contains a pair of any two letters that 
   appears at least twice in the string without overlapping, like 
   xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
 - It contains at least one letter which repeats 
   with exactly one letter between them, like xyx, abcdefeghi (efe), 
   or even aaa. */
fn part_2(vec: &Vec<Vec<char>>) -> usize {
    // Final number of nice strings
    let mut nice = 0;

    for line in vec {
        // Each line might be nice if it gets enough points
        let mut separated = false;
        let mut doubles = false;
        let mut pairs: &str = "";
        let mut separated_pair: String = String::new();

        // Checks for repeating pairs of characters
        let mut pair_counts: HashMap<&str, usize> = HashMap::new();
        let line_str: String = line.iter().collect();
        // Slices the string to get a pair
        // Counts pair occurences starting at 1
        // If the pair is part of a triple like aaa, the index
        // advances by 2, otherwise it advances by 1
        let mut i = 0;
        while i < line_str.len() - 1 {
            let pair = &line_str[i..i + 2];
            *pair_counts.entry(pair).or_insert(1) += 1;
            if i + 2 < line_str.len() && line_str[i..=i + 1] == line_str[i + 1..=i + 2] {
                i += 2;
            } else {
                i += 1;
            }
        }       
        // Checks for multiple occurences
        for (pair, count) in &pair_counts {
            if *count > 2 {
                pairs = pair; // For print debugging
                doubles = true;
                break // One is all you need
            }
        }

        // Checks for separated pairs
        for (i, char) in line.iter().enumerate() {
            if i == 0 || i == 1 { 
                continue
            };
            if *char == line[i - 2] { 
                separated = true;
                // For print debugging
                separated_pair = format!("{}{}{}", 
                    char, 
                    line[i-1], 
                    line[i-2]
                );
                break // One is all you need
            }
        }

        // The string is nice if both conditions are true
        if separated == true && doubles == true { 
            eprintln!("\"{:?}\" contains {} : {}", line, separated_pair, pairs);
            nice += 1 
        };
    }

    return nice;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d05_1() {

        // ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), 
        // a double letter (...dd...), and none of the disallowed substrings.
        let a: Vec<char> = "ugknbfddgicrmopn".chars().collect();
        
        //aaa is nice because it has at least three vowels and a double letter, 
        //even though the letters used by different rules overlap.
        let b: Vec<char> = "aaa".chars().collect();

        //jchzalrnumimnmhp is naughty because it has no double letter.
        let c: Vec<char> = "jchzalrnumimnmhp".chars().collect();

        //haegwjzuvuyypxyu is naughty because it contains the string xy.
        let d: Vec<char> = "haegwjzuvuyypxyu".chars().collect();

        //dvszwmarrgswjxmb is naughty because it contains only one vowel.
        let e: Vec<char> = "dvszwmarrgswjxmb".chars().collect();

        let all = vec![a, b, c, d, e];
        let result = part_1(&all);
        assert_eq!(result, 2);
    }

    #[test]
    fn d05_2() {

        // qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) 
        // and a letter that repeats with exactly one letter between them (zxz).
        let a: Vec<char> = "qjhvhtzxzqqjkmpb".chars().collect();

        // xxyxx is nice because it has a pair that appears twice and a letter 
        // that repeats with one between, even though the letters used by each rule overlap.
        let b: Vec<char> = "xxyxx".chars().collect();

        // uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat 
        // with a single letter between them.
        let c: Vec<char> = "uurcxstgmygtbstg".chars().collect();

        // ieodomkazucvgmuy is naughty because it has a repeating letter with 
        // one between (odo), but no pair that appears twice.
        let d: Vec<char> = "ieodomkazucvgmuy".chars().collect();

        // Nice
        let e: Vec<char> = "aaaa".chars().collect();

        // Nice
        let f: Vec<char> = "abab".chars().collect();

        // Naughty
        let g: Vec<char> = "aaa".chars().collect();

        let all = vec![a, b, c, d, e, f, g];
        let result = part_2(&all);
        assert_eq!(result, 4);

    }
}
