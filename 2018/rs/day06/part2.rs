mod coordinate_map;

use coordinate_map::CoordinateMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut map = CoordinateMap::new();

    BufReader::new(File::open("day06/input.txt").unwrap())
        .lines()
        .for_each(|line| {
            map.add(line.unwrap());
        });

    let mut safe_squares = 0;

    for i in 0..=map.rightmost.unwrap() {
        for j in 0..=map.topmost.unwrap() {
            safe_squares += match map
                .coords
                .iter()
                .fold(0, |sum, coord| sum + coord.distance_to(i, j))
            {
                n if n < 10000 => 1,
                _ => 0,
            }
        }
    }

    println!("The result is {}", safe_squares);
}
