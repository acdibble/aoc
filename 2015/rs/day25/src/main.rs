use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn parse_coords(input: String) -> (usize, usize) {
    let mut x = String::new();
    let mut y = String::new();

    let mut it = input.split_ascii_whitespace();

    while let Some(string) = it.next() {
        if string == "column" {
            let mut chars = it.next().unwrap().chars();
            chars.next_back();
            x = chars.as_str().to_owned();
        } else if string == "row" {
            let mut chars = it.next().unwrap().chars();
            chars.next_back();
            y = chars.as_str().to_owned();
        }
    }

    (x.parse().unwrap(), y.parse().unwrap())
}

fn part_one(x: usize, y: usize) -> u64 {
    let code_count = (0..(x + y - 1)).sum::<usize>() + x;

    (0..code_count - 1).fold(20151125u64, |acc, _| (acc * 252533) % 33554393)
}

fn time_it<F>(fun: F)
where
    F: Fn() -> (),
{
    let start = SystemTime::now();
    fun();
    println!("Time elapsed: {} ms", start.elapsed().unwrap().as_millis())
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    let (x, y) = parse_coords(input);

    time_it(|| println!("part 1: {}", part_one(x, y)));

    Ok(())
}
