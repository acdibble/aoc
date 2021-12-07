use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn simple_fuel_consumption(crabs: &Vec<i32>, position: i32) -> i32 {
    crabs
        .iter()
        .fold(0, |acc, crab| acc + (position - crab).abs())
}

fn summation_fuel_consumption(crabs: &Vec<i32>, position: i32) -> i32 {
    crabs.iter().fold(0, |acc, crab| {
        let distance = (position - crab).abs();
        acc + (distance * (distance + 1)) / 2
    })
}

fn binary_search(
    crabs: &Vec<i32>,
    mut left: i32,
    mut right: i32,
    fuel_fn: fn(&Vec<i32>, i32) -> i32,
) -> i32 {
    let mut lowest_amount = i32::MAX;

    while left < right {
        let mid = (left + right) / 2;

        let current_result = fuel_fn(&crabs, mid);
        let right_result = fuel_fn(&crabs, mid + 1);

        lowest_amount = lowest_amount.min(current_result);
        lowest_amount = lowest_amount.min(current_result);

        if right_result >= current_result {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    lowest_amount
}

fn solve(input: &str) -> (i32, i32) {
    let crabs: Vec<i32> = input
        .split(',')
        .map(|string| string.parse().unwrap())
        .collect();

    let left = *crabs.iter().min().unwrap();
    let right = *crabs.iter().max().unwrap();

    (
        binary_search(&crabs, left, right, simple_fuel_consumption),
        binary_search(&crabs, left, right, summation_fuel_consumption),
    )
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
