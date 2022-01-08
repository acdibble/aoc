use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl std::str::FromStr for Direction {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        let direction = match string {
            "n" => Direction::North,
            "ne" => Direction::NorthEast,
            "nw" => Direction::NorthWest,
            "s" => Direction::South,
            "se" => Direction::SouthEast,
            "sw" => Direction::SouthWest,
            _ => return Err(()),
        };

        Ok(direction)
    }
}

#[derive(Default)]
struct Grid {
    q: i32,
    r: i32,
    s: i32,
}

impl Grid {
    fn new() -> Self {
        Default::default()
    }

    fn distance_from_origin(&self) -> i32 {
        self.q.abs().max(self.r.abs()).max(self.s.abs())
    }

    fn step(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                self.r -= 1;
                self.s += 1;
            }
            Direction::NorthEast => {
                self.q += 1;
                self.r -= 1;
            }
            Direction::SouthEast => {
                self.q += 1;
                self.s -= 1;
            }
            Direction::South => {
                self.r += 1;
                self.s -= 1;
            }
            Direction::SouthWest => {
                self.q -= 1;
                self.r += 1;
            }
            Direction::NorthWest => {
                self.q -= 1;
                self.s += 1;
            }
        }
    }
}

fn solve(input: &str) -> (i32, i32) {
    let mut grid = Grid::new();
    let mut max_distance = 0;

    for string in input.split(',') {
        match string.parse() {
            Ok(dir) => grid.step(dir),
            _ => unreachable!(string),
        };

        max_distance = max_distance.max(grid.distance_from_origin());
    }

    (grid.distance_from_origin(), max_distance)
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

    time_it(|| println!("parts (1, 2): {:?}", solve(&input)));

    Ok(())
}
