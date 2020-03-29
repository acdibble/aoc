use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let reader = BufReader::new(File::open("day01/input.txt").unwrap());

    let mut total = 0;

    for line in reader.lines() {
        total += line.unwrap().parse::<i32>().unwrap();
    }

    println!("Total is: {}", total);
}
