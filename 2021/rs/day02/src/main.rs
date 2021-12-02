use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(input: &str) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();

        let direction = parts.next();

        let amount: i32 = parts.next().unwrap().parse().unwrap();

        match direction {
            Some("forward") => horizontal += amount,
            Some("up") => depth -= amount,
            Some("down") => depth += amount,
            _ => unreachable!(),
        }
    }

    depth * horizontal
}

fn part_two(input: &str) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();

        let direction = parts.next();

        let amount: i32 = parts.next().unwrap().parse().unwrap();

        match direction {
            Some("forward") => {
                horizontal += amount;
                depth += aim * amount;
            }
            Some("up") => aim -= amount,
            Some("down") => aim += amount,
            _ => unreachable!(),
        }
    }

    depth * horizontal
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
