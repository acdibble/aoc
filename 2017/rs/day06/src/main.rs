use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn solve(input: &str) -> (i32, i32) {
    let mut banks: Vec<i32> = input
        .split_ascii_whitespace()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut iterations = 0;
    let mut seen = HashMap::new();

    while !seen.contains_key(&banks) {
        seen.insert(banks.clone(), iterations);
        iterations += 1;

        let mut to_redistribute = *banks.iter().max().unwrap();
        let mut index = banks.iter().position(|&v| v == to_redistribute).unwrap();

        banks[index] = 0;

        while to_redistribute != 0 {
            index = (index + 1) % banks.len();
            banks[index] += 1;
            to_redistribute -= 1;
        }
    }

    let loop_size = iterations - seen.get(&banks).unwrap();
    (iterations, loop_size)
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

    time_it(|| println!("parts (1, 2): {:?}", solve(&input)));

    Ok(())
}
