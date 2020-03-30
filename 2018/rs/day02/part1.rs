use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn count_occurrences(id: String) -> (i32, i32) {
    let mut map = HashMap::<char, i32>::new();

    for c in id.chars() {
        let counter = map.entry(c).or_insert(0);
        *counter += 1;
    }

    let mut result = (0, 0);

    for (_, v) in map.iter() {
        match v {
            2 => result.0 = 1,
            3 => result.1 = 1,
            _ => (),
        }
    }

    return result;
}

fn main() {
    let (double, triple): (i32, i32) = BufReader::new(File::open("day02/input.txt").unwrap())
        .lines()
        .fold((0, 0), |(d, t), line| {
            let result = count_occurrences(line.unwrap().parse().unwrap());
            (d + result.0, t + result.1)
        });

    println!("dubs {}, trips {}", double, triple);
    println!("product {}", double * triple);
}
