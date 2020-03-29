use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let nums: Vec<i32> = BufReader::new(File::open("day01/input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut frequencies = HashSet::new();
    let mut total = 0;

    loop {
        for num in nums.iter() {
            total += num;

            if !frequencies.insert(total) {
                println!("First repeated frequency is {}", total);
                return;
            }
        }
    }
}
