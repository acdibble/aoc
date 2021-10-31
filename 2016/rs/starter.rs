use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(input: &String) -> i32 {
    0
}

fn part_two(input: &String) -> i32 {
    0
}

fn time_it<F>(fun: F)
where
    F: Fn() -> (),
{
    let start = SystemTime::now();
    fun();
    println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros())
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("part 1: {}", part_one(&input)));
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}

