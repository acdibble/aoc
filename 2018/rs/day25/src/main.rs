use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Point(i32, i32, i32, i32);

impl Point {
    fn distance(&self, other: &Self) -> i32 {
        (self.0 - other.0).abs()
            + (self.1 - other.1).abs()
            + (self.2 - other.2).abs()
            + (self.3 - other.3).abs()
    }

    fn from_str(line: &str) -> Self {
        let mut parts = line.split(',');
        Self(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        )
    }
}

fn part_one(input: &str) -> usize {
    let mut points: Vec<_> = input.lines().map(Point::from_str).collect();

    let mut count = 0;

    let mut constellation = vec![];
    while points.len() != 0 {
        let mut j = 0;
        constellation.clear();
        constellation.push(points.pop().unwrap());

        while j < constellation.len() {
            let mut i = 0;
            while i < points.len() {
                if points[i].distance(&constellation[j]) <= 3 {
                    let point = points.swap_remove(i);
                    constellation.push(point);
                } else {
                    i += 1;
                }
            }
            j += 1;
        }

        count += 1;
    }

    count
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

    Ok(())
}
