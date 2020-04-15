use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default)]
struct Coord {
    x: i32,
    y: i32,
    count: i32,
}

impl Coord {
    fn from_string(string: String) -> Coord {
        let mut coord: Coord = Default::default();
        let mut x_set = false;
        for part in string.split(", ") {
            if x_set {
                coord.y = part.parse().unwrap();
            } else {
                coord.x = part.parse().unwrap();
            }
            x_set = true;
        }
        coord
    }

    fn distance_to(&self, x: i32, y: i32) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

#[derive(Debug, Default)]
struct CoordMap {
    leftmost: Option<i32>,
    rightmost: Option<i32>,
    topmost: Option<i32>,
    bottommost: Option<i32>,
    coords: Vec<Coord>,
}

impl CoordMap {
    fn new() -> CoordMap {
        Default::default()
    }

    fn add(&mut self, coord: Coord) -> &mut CoordMap {
        self.coords.push(coord);
        self
    }
}

fn parse_line(map: &mut CoordMap, line: Result<String, std::io::Error>) -> &mut CoordMap {
    let coord = Coord::from_string(line.unwrap());

    map.leftmost = if map.leftmost.unwrap_or(999999) > coord.x {
        Some(coord.x)
    } else {
        map.leftmost
    };

    map.topmost = if map.topmost.unwrap_or(0) < coord.y {
        Some(coord.x)
    } else {
        map.topmost
    };

    map.bottommost = if map.bottommost.unwrap_or(999999) > coord.y {
        Some(coord.x)
    } else {
        map.bottommost
    };

    map.rightmost = if map.rightmost.unwrap_or(0) < coord.x {
        Some(coord.x)
    } else {
        map.rightmost
    };

    map.add(coord)
}

fn main() {
    let mut map: CoordMap = CoordMap::new();

    let parsed = BufReader::new(File::open("day06/input.txt").unwrap())
        .lines()
        .fold(&mut map, parse_line);

    for i in parsed.leftmost.unwrap()..=parsed.rightmost.unwrap() {
        for j in parsed.bottommost.unwrap()..=parsed.topmost.unwrap() {
            let mut tie = false;
            let mut shortest_distance = 999999;
            let mut closest: Option<&mut Coord> = None;

            for coord in parsed.coords.iter_mut() {
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
        parsed.coords.iter().max_by_key(|c| c.count).unwrap().count
    );
}
