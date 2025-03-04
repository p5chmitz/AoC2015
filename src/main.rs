mod day;
use day::{day01, day02, day03, day04, day05};

fn main() {
    println!("My hacky attempts at Advent of Code™ 2015! 🎄🎁\nhttps://adventofcode.com/2015");

    println!("Day 1:");
    let d3 = day01::runner();
    println!(
        "\tThe elevator first goes into the basement on the {} iteration.",
        d3.0
    );
    println!("\tThe resultant floor is: {}", d3.1);

    println!("Day 2:");
    let d2 = day02::runner();
    println!("\tWrapping paper area: {}", d2.0);
    println!("\tTotal ribbon length: {}", d2.1);

    println!("Day 3:");
    let d3 = day03::runner();
    println!("\tHouses that get at least one present: {}", d3.0);
    println!("\tUnique hits with Robo Santa™: {}", d3.1);

    println!("Day 4:");
    println!("\t(Runner takes too long to process)");
    //let d4 = day04::runner();
    //println!("\tKey suffix for 5 0s: {}", d4.0);
    //println!("\tKey suffix for 6 0s: {}", d4.1);

    println!("Day 5:");
    let d5 = day05::runner();
    println!("\tNumber of \"nice\" strings: {}", d5.0);
    println!("\tNumber of \"_really_ nice\" strings: {}", d5.1);
}
