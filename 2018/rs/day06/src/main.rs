use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
    count: i32,
}

impl Coordinate {
    fn from_string(string: &str) -> Self {
        let mut parts = string.split(", ");
        Self {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            count: 0,
        }
    }

    #[allow(dead_code)]
    fn distance_to(&self, x: i32, y: i32) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

#[derive(Debug, Default)]
struct CoordinateMap {
    leftmost: Option<i32>,
    rightmost: Option<i32>,
    topmost: Option<i32>,
    bottommost: Option<i32>,
    coords: Vec<Coordinate>,
}

impl CoordinateMap {
    fn new() -> Self {
        Default::default()
    }

    fn add(&mut self, coord_str: &str) {
        let coord = Coordinate::from_string(coord_str);

        self.leftmost = if self.leftmost.unwrap_or(999999) > coord.x {
            Some(coord.x)
        } else {
            self.leftmost
        };

        self.topmost = if self.topmost.unwrap_or(0) < coord.y {
            Some(coord.x)
        } else {
            self.topmost
        };

        self.bottommost = if self.bottommost.unwrap_or(999999) > coord.y {
            Some(coord.x)
        } else {
            self.bottommost
        };

        self.rightmost = if self.rightmost.unwrap_or(0) < coord.x {
            Some(coord.x)
        } else {
            self.rightmost
        };

        self.coords.push(coord);
    }
}

fn part_one(input: &str) -> i32 {
    let mut map = input.lines().fold(CoordinateMap::new(), |mut acc, line| {
        acc.add(line);
        acc
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

    map.coords.iter().max_by_key(|c| c.count).unwrap().count
}

fn part_two(input: &str) -> i32 {
    let map = input.lines().fold(CoordinateMap::new(), |mut acc, line| {
        acc.add(line);
        acc
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

    safe_squares
}

fn time_it<F, T>(fun: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();
    let result = fun();
    println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros());
    result
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("part 1: {}", part_one(&input)));
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}
