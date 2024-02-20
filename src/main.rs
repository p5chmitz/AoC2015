mod day;
use day::{one, three, two};

fn main() {
    println!("My hacky attempts at Advent of Code™ 2015! 🎄🎁\nhttps://adventofcode.com/2015");

    println!("Day 1:");
    let d3 = one::runner();
    println!(
        "\tThe elevator first goes into the basement on the {} iteration.",
        d3.0
    );
    println!("\tThe resultant floor is: {}", d3.1);

    println!("Day 2:");
    let d2 = two::runner();
    println!("\tWrapping paper area: {}", d2.0);
    println!("\tTotal ribbon length: {}", d2.1);

    println!("Day 3:");
    let d3 = three::runner();
    println!("\tHouses that get at least one present: {}", d3.0);
    println!("\tUnique hits with Robo Santa™: {}", d3.1);
}
