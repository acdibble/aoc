use std::cmp::Ordering;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
struct Scanner {
    index: i32,
    range: i32,
    half_circuit: i32,
}

impl Scanner {
    fn new(index: i32, range: i32) -> Self {
        Self {
            index,
            range,
            half_circuit: range - 1,
        }
    }

    fn location_after_delay(&self, delay: i32) -> i32 {
        let current_position = (self.index + delay) % (self.half_circuit * 2);

        match current_position.cmp(&self.half_circuit) {
            Ordering::Equal | Ordering::Less => current_position,
            Ordering::Greater => self.range - (current_position - self.half_circuit),
        }
    }

    fn location(&self) -> i32 {
        self.location_after_delay(0)
    }

    fn severity(&self) -> i32 {
        self.index * self.range
    }
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut scanners = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(": ");
        let index: i32 = parts.next().unwrap().parse().unwrap();
        let depth = parts.next().unwrap().parse().unwrap();

        scanners.push(Scanner::new(index, depth))
    }

    scanners
}

fn part_one(scanners: &Vec<Scanner>) -> i32 {
    let mut severity = 0;

    for scanner in scanners {
        if scanner.location() == 0 {
            severity += scanner.severity()
        }
    }

    severity
}

#[inline(always)]
fn try_delay(scanners: &Vec<Scanner>, delay: i32) -> Result<(), ()> {
    for scanner in scanners {
        if scanner.location_after_delay(delay) == 0 {
            return Err(());
        }
    }

    Ok(())
}

fn part_two(scanners: &Vec<Scanner>) -> i32 {
    for delay in 0.. {
        match try_delay(scanners, delay) {
            Ok(_) => return delay,
            _ => (),
        }
    }

    unreachable!()
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
    let scanners = parse_input(&input);

    time_it(|| println!("part 1: {}", part_one(&scanners)));
    time_it(|| println!("part 2: {}", part_two(&scanners)));

    Ok(())
}
