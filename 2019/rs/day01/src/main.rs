use std::{env, fs, path::Path, time::SystemTime};

fn part_one(input: &Vec<i32>) -> i32 {
    input.iter().map(|x| x / 3 - 2).sum()
}

fn calculate_cost(value: i32, total: i32) -> i32 {
    let result = value / 3 - 2;

    if result <= 0 {
        total
    } else {
        calculate_cost(result, total + result)
    }
}

fn part_two(input: &Vec<i32>) -> i32 {
    input.iter().map(|x| calculate_cost(*x, 0)).sum()
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
    let numbers: Vec<i32> = input.split('\n').flat_map(|s| s.parse::<i32>()).collect();

    time_it(|| println!("part 1: {}", part_one(&numbers)));
    time_it(|| println!("part 1: {}", part_two(&numbers)));

    Ok(())
}
