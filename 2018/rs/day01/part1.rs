use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let total: i32 = BufReader::new(File::open("day01/input.txt").unwrap())
        .lines()
        .fold(0, |acc, line| acc + line.unwrap().parse::<i32>().unwrap());

    println!("Total is: {}", total);
}
