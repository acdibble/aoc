use std::env;
use std::fs;
use std::path::Path;

fn part_one(target: usize) -> usize {
    let max_size = target / 10;

    let mut houses = vec![0; max_size];

    for i in 1..max_size {
        for j in (i..max_size).step_by(i) {
            houses[j] += i * 10;
        }
    }

    houses.into_iter().position(|v| v >= target).unwrap()
}

fn part_two(target: usize) -> usize {
    let max_size = target / 10;

    let mut houses = vec![0; max_size];

    for i in 1..max_size {
        for j in 1..=50 {
            let index = j * i;
            if index >= max_size {
                break;
            }
            houses[index] += i * 11;
        }
    }

    houses.into_iter().position(|v| v >= target).unwrap()
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input: usize = fs::read_to_string(file_path)?.parse().unwrap();

    println!("part 1: {}", part_one(input));
    println!("part 2: {}", part_two(input));

    Ok(())
}
