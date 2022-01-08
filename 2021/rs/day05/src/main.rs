use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::time::SystemTime;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut parts = string.split(',');

        if let (Some(x), Some(y)) = (parts.next(), parts.next()) {
            if let (Ok(x), Ok(y)) = (x.parse(), y.parse()) {
                return Ok(Self { x, y });
            }
        }

        Err(())
    }
}

struct LineIter {
    current: Point,
    end: Point,
    x_diff: i32,
    y_diff: i32,
    done: bool,
}

impl Iterator for LineIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.done {
            return None;
        }

        self.done = self.current == self.end;
        let out = (self.current.x, self.current.y);

        self.current.x += self.x_diff;
        self.current.y += self.y_diff;

        Some(out)
    }
}

fn get_diff(ordering: Ordering) -> i32 {
    match ordering {
        Ordering::Greater => -1,
        Ordering::Equal => 0,
        Ordering::Less => 1,
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn points(&self) -> LineIter {
        LineIter {
            current: self.start,
            x_diff: get_diff(self.start.x.cmp(&self.end.x)),
            y_diff: get_diff(self.start.y.cmp(&self.end.y)),
            end: self.end,
            done: false,
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut parts = string.split(" -> ");

        if let (Some(start), Some(end)) = (parts.next(), parts.next()) {
            let start = start.parse()?;
            let end = end.parse()?;
            return Ok(Self { start, end });
        }

        Err(())
    }
}

fn solve(input: &str) -> (i32, i32) {
    let mut grid = HashMap::<(i32, i32), (i32, i32)>::new();

    let mut part_one = 0;
    let mut part_two = 0;

    for line in input.lines().map(|l| l.parse::<Line>().unwrap()) {
        let is_diagonal = line.start.x != line.end.x && line.start.y != line.end.y;
        for p in line.points() {
            let entry = grid.entry(p).or_default();
            let mut current = *entry;
            current.1 += 1;

            if !is_diagonal {
                current.0 += 1;
                if current.0 == 2 {
                    part_one += 1;
                }
            }

            if current.1 == 2 {
                part_two += 1;
            }

            *entry = current;
        }
    }

    (part_one, part_two)
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

    time_it(|| println!("part (1, 2): {:?}", solve(&input)));

    Ok(())
}
