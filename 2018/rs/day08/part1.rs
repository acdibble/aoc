use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_node(numbers: &mut Vec<i32>) -> i32 {
    let child_count = numbers.remove(0);
    let metadata_count = numbers.remove(0);
    let mut current_total = 0;
    for _ in 0..child_count {
        current_total += parse_node(numbers);
    }

    for _ in 0..metadata_count {
        current_total += numbers.remove(0);
    }

    return current_total;
}

fn main() {
    let mut numbers = Vec::<i32>::new();
    let mut buf = Vec::<u8>::new();

    let mut reader = BufReader::new(File::open("input.txt").unwrap());

    loop {
        let length = reader.read_until(b' ', &mut buf).unwrap();

        if length == 0 {
            break;
        }

        let num = String::from_utf8(buf.clone()).unwrap();

        numbers.push(num.trim_end().parse().unwrap());

        buf.clear();
    }

    let total = parse_node(&mut numbers);
    println!("total: {}", total);
}
