mod coordinate_map;

use coordinate_map::{Coordinate, CoordinateMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut map = CoordinateMap::new();

    BufReader::new(File::open("day06/input.txt").unwrap())
        .lines()
        .for_each(|l| {
            map.add(l.unwrap());
        });

    for i in map.leftmost.unwrap()..=map.rightmost.unwrap() {
        for j in map.bottommost.unwrap()..=map.topmost.unwrap() {
            let mut tie = false;
            let mut shortest_distance = 999999;
            let mut closest: Option<&mut Coordinate> = None;

            for coord in map.coords.iter_mut() {
                let current_distance = coord.distance_to(i, j);
                if current_distance < shortest_distance {
                    shortest_distance = current_distance;
                    closest = Some(coord);
                    tie = false;
                } else if shortest_distance == current_distance {
                    tie = true;
                }
            }

            if !tie {
                closest.unwrap().count += 1;
            }
        }
    }

    println!(
        "The result is {}",
        map.coords.iter().max_by_key(|c| c.count).unwrap().count
    );
}
