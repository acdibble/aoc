use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(input: &str) -> i32 {
    let mut offsets: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut steps = 0;
    let mut pc = 0i32;

    while let Some(offset) = offsets.get_mut(pc as usize) {
        steps += 1;
        pc += *offset;
        *offset += 1;
    }

    steps
}

fn part_two(input: &str) -> i32 {
    let mut offsets: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut steps = 0;
    let mut pc = 0i32;

    while let Some(offset) = offsets.get_mut(pc as usize) {
        steps += 1;
        pc += *offset;
        *offset += if *offset >= 3 { -1 } else { 1 };
    }

    steps
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
