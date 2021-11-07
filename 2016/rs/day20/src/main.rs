use std::cmp;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug)]
struct Range(u32, u32);

impl Range {
    fn from(string: &str) -> Self {
        let mut parts = string.split('-');

        Self(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        )
    }

    fn contains(&self, value: u32) -> bool {
        self.0 <= value && self.1 >= value
    }

    fn lt(&self, value: u32) -> bool {
        self.1 < value
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.contains(other.0) || self.contains(other.1)
    }

    fn abuts(&self, other: &Range) -> bool {
        if self.1 == u32::MAX || other.1 == u32::MAX {
            return false;
        }

        self.1 + 1 == other.0 || other.1 + 1 == self.0
    }

    fn merge(self, other: Self) -> Self {
        Self(cmp::min(self.0, other.0), cmp::max(self.1, other.1))
    }
}

fn part_one(input: &String) -> (u32, Vec<Range>) {
    let mut ranges: Vec<_> = input.lines().map(Range::from).collect();
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut lowest = 0;

    for range in &ranges {
        if range.contains(lowest) {
            lowest = range.1 + 1;
        } else if range.lt(lowest) {
            continue;
        } else {
            break;
        }
    }

    (lowest, ranges)
}

fn part_two(ranges: &Vec<Range>) -> u32 {
    let mut allowed = 0;
    let mut iterator = ranges.iter().copied();
    let mut current = iterator.next().unwrap();

    while let Some(next) = iterator.next() {
        if current.overlaps(&next) || current.abuts(&next) {
            current = current.merge(next);
        } else {
            allowed += next.0 - current.1 - 1;
            current = next;
        }
    }

    allowed
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

    let ranges = time_it(|| {
        let (lowest, ranges) = part_one(&input);
        println!("part 1: {}", lowest);
        ranges
    });
    time_it(|| println!("part 2: {}", part_two(&ranges)));

    Ok(())
}
