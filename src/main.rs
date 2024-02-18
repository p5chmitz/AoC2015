mod day;
use day::{
    one, 
    two,
    three,
};

fn main() {
    println!("Day 1:");
    print!("\t");
    one::runner();
    
    println!("Day 2:");
    let d2 = two::runner();
    println!("\tWrapping paper area: {}\n\tTotal ribbon length: {}", d2.0, d2.1);

    println!("Day 3:");
    let d3 = three::runner();
    println!("\tHouses that get at least one present: {}", d3.0);

}
