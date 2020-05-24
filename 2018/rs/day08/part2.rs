use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_node(numbers: &mut Vec<i32>) -> i32 {
    let child_count = numbers.remove(0);
    let metadata_count = numbers.remove(0);
    let mut current_total = 0;
    let mut child_values = Vec::<i32>::new();

    for _ in 0..child_count {
        child_values.push(parse_node(numbers));
    }

    for _ in 0..metadata_count {
        let current = numbers.remove(0);
        if child_count > 0 {
            if current > 0 && current <= child_count {
                current_total += child_values[(current - 1) as usize];
            }
        } else {
            current_total += current;
        }
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
