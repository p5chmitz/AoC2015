#![allow(warnings)]

use std::fs::File;
use std::io::Read;

pub fn runner() -> (i32, i32) {
    // Part one
    let mut results = (0, 0);
    let key = String::from("iwrupvqb");
    let pattern = String::from("00000");
    println!("\tProcessing hashes; this may take some time...");
    results.0 = core_logic(&key, &pattern, 0, 5);
    println!("\tFirst hash processed!");
    // Part two
    let pattern = String::from("000000");
    results.1 = core_logic(&key, &pattern, 0, 6);
    println!("\tSecond hash processed!");
    results
}

fn core_logic(key: &str, pattern: &str, lower: usize, upper: usize) -> i32 {
    let mut num: i32 = 0;
    let mut payload = String::new();
    let mut digest = String::new();
    let mut repeat = true;
    while repeat {
        payload = format!("{}{}", key, num);
        digest = hash(&payload);
        if digest[lower..upper].contains(&pattern) {
            repeat = false;
            return num;
        } else {
            num += 1;
        }
    }
    num
}

fn hash(s: &str) -> String {
    let digest = md5::compute(s.as_bytes());
    let hash_str = digest
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();
    //println!("Core logic hash str: {}", hash_str);
    hash_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d04_1() {
        let s = String::from("abcdef609043");
        let hash_val = hash(&s);
        println!("Hash of test val abcdef609043: {}", hash_val);
    }
    #[test]
    #[ignore]
    fn d04_2() {
        let key = String::from("abcdef");
        let pattern = String::from("00000");
        let result = core_logic(&key, &pattern, 0, 5);
        let test = 609043;
        assert_eq!(test, result);
    }
    #[test]
    #[ignore]
    fn d04_3() {
        let key = String::from("pqrstuv");
        let pattern = String::from("00000");
        let result = core_logic(&key, &pattern, 0, 5);
        let test = 1048970;
        assert_eq!(test, result);
    }
}
