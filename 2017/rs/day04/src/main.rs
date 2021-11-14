use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(input: &str) -> i32 {
    let mut frequency_counts = HashMap::<&str, i32>::new();
    let mut valid_count = 0;

    for line in input.lines() {
        frequency_counts.clear();
        valid_count += 1;

        for word in line.split_ascii_whitespace() {
            let entry = frequency_counts.entry(word).or_default();
            if *entry != 0 {
                valid_count -= 1;
                break;
            }
            *entry += 1;
        }
    }

    valid_count
}

fn part_two(input: &str) -> i32 {
    let mut frequency_counts = HashMap::<String, i32>::new();
    let mut valid_count = 0;

    for line in input.lines() {
        frequency_counts.clear();
        valid_count += 1;

        for word in line.split_ascii_whitespace() {
            let mut chars = word.chars().collect::<Vec<char>>();
            chars.sort_by(|a, b| a.cmp(&b));

            let entry = frequency_counts
                .entry(chars.into_iter().collect())
                .or_default();
            if *entry != 0 {
                valid_count -= 1;
                break;
            }
            *entry += 1;
        }
    }

    valid_count
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
