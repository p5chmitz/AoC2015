mod day;
use day::{one, two};

fn main() {
    println!("Day 1:");
    print!("\t");
    one::runner();
    println!("Day 2:");
    let d2 = two::runner();
    println!("\tArea: {}\n\tLength: {}", d2.0, d2.1);
}
