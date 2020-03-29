use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn parse_frequencies(reader: BufReader<File>) -> Vec::<i32> {
    let mut nums = Vec::<i32>::new();

    for line in reader.lines() {
        nums.push(line.unwrap().parse::<i32>().unwrap());
    }

    nums
}

fn main() {
    let reader = BufReader::new(File::open("day01/input.txt").unwrap());
    let nums = parse_frequencies(reader);
    let mut frequencies = HashSet::<i32>::new();
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
