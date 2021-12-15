use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(input: &str) -> i32 {
    fn parse_node(numbers: &mut Vec<i32>) -> i32 {
        let child_count = numbers.pop().unwrap();
        let metadata_count = numbers.pop().unwrap();
        let mut current_total = 0;
        for _ in 0..child_count {
            current_total += parse_node(numbers);
        }

        for _ in 0..metadata_count {
            current_total += numbers.pop().unwrap();
        }

        current_total
    }

    let mut numbers = input
        .split_ascii_whitespace()
        .flat_map(|n| n.parse())
        .rev()
        .collect();

    parse_node(&mut numbers)
}

fn part_two(input: &str) -> i32 {
    fn parse_node(numbers: &mut Vec<i32>) -> i32 {
        let child_count = numbers.pop().unwrap();
        let metadata_count = numbers.pop().unwrap();
        let mut current_total = 0;
        let mut child_values = Vec::<i32>::new();

        for _ in 0..child_count {
            child_values.push(parse_node(numbers));
        }

        for _ in 0..metadata_count {
            let current = numbers.pop().unwrap();
            if child_count > 0 {
                if current > 0 && current <= child_count {
                    current_total += child_values[(current - 1) as usize];
                }
            } else {
                current_total += current;
            }
        }

        current_total
    }

    let mut numbers = input
        .split_ascii_whitespace()
        .flat_map(|n| n.parse())
        .rev()
        .collect();

    parse_node(&mut numbers)
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
