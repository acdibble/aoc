use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(measurements: &Vec<i32>) -> i32 {
    measurements.windows(2).fold(0, |acc, slice| {
        acc + if slice[0] < slice[1] { 1 } else { 0 }
    })
}

fn part_two(measurements: &Vec<i32>) -> i32 {
    let mut previous = 0;
    let mut increases = -1;

    for window in measurements.windows(3) {
        let sum = window.iter().sum();
        if sum > previous {
            increases += 1;
        }
        previous = sum;
    }

    increases
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
    let measurements = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();

    time_it(|| println!("part 1: {}", part_one(&measurements)));
    time_it(|| println!("part 2: {}", part_two(&measurements)));

    Ok(())
}
