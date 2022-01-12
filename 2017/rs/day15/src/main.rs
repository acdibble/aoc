use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
struct Generator {
    factor: u64,
    current: u64,
    multiple: u64,
}

impl Generator {
    fn new(factor: u64, multiple: u64, string: &str) -> Self {
        let mut parts = string.split_ascii_whitespace();
        parts.next();
        parts.next();
        parts.next();
        parts.next();
        Self {
            factor,
            multiple,
            current: parts.next().unwrap().parse().unwrap(),
        }
    }
}

impl Iterator for Generator {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current = (self.current * self.factor) % 2147483647;
            if self.multiple == 1 || self.current % self.multiple == 0 {
                return Some(self.current as u16);
            }
        }
    }
}

fn part_one(input: &str) -> i32 {
    let mut lines = input.lines();

    Generator::new(16807, 1, lines.next().unwrap())
        .zip(Generator::new(48271, 1, lines.next().unwrap()))
        .take(40_000_000)
        .map(|(a, b)| if a == b { 1 } else { 0 })
        .sum()
}

fn part_two(input: &str) -> i32 {
    let mut lines = input.lines();

    Generator::new(16807, 4, lines.next().unwrap())
        .zip(Generator::new(48271, 8, lines.next().unwrap()))
        .take(5_000_000)
        .map(|(a, b)| if a == b { 1 } else { 0 })
        .sum()
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
