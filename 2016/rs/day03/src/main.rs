use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn is_valid(a: i32, b: i32, c: i32) -> bool {
    a + b > c && b + c > a && a + c > b
}

fn part_one(input: &String) -> i32 {
    input.lines().fold(0, |acc, line| {
        let mut it = line.split_ascii_whitespace();
        acc + is_valid(
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        ) as i32
    })
}

fn part_two(input: &String) -> i32 {
    let mut line_it = input.lines();
    let mut result = 0;

    loop {
        let first_line = line_it.next();
        if first_line.is_none() {
            break;
        }
        let mut first_line = first_line.unwrap().split_ascii_whitespace();
        let mut second_line = line_it.next().unwrap().split_ascii_whitespace();
        let mut third_line = line_it.next().unwrap().split_ascii_whitespace();

        for _ in 0..3 {
            result += is_valid(
                first_line.next().unwrap().parse().unwrap(),
                second_line.next().unwrap().parse().unwrap(),
                third_line.next().unwrap().parse().unwrap(),
            ) as i32
        }
    }

    result
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
