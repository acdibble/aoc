use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(input: &str) -> i32 {
    input
        .lines()
        .fold(0, |acc, line| acc + line.parse::<i32>().unwrap())
}

fn part_two(input: &str) -> i32 {
    let nums: Vec<i32> = input.lines().flat_map(|line| line.parse()).collect();

    let mut frequencies = HashSet::new();
    let mut total = 0;

    loop {
        for num in nums.iter() {
            total += num;

            if !frequencies.insert(total) {
                return total;
            }
        }
    }
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
