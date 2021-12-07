use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn solve(input: &str) -> (i32, i32) {
    let crabs: Vec<i32> = input
        .split(',')
        .map(|string| string.parse().unwrap())
        .collect();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let mut part_one = i32::MAX;
    let mut part_two = i32::MAX;

    for position in min..=max {
        let mut current_part_one = 0;
        let mut current_part_two = 0;

        for crab in &crabs {
            let distance = (position - crab).abs();
            current_part_one += distance;
            current_part_two += (distance * (distance + 1)) / 2;
        }

        if current_part_one > part_one && current_part_two > part_two {
            break;
        }

        part_one = part_one.min(current_part_one);
        part_two = part_two.min(current_part_two);
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
