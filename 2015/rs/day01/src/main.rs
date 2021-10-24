use std::env;
use std::fs;
use std::path::Path;

fn part_one(data: &String) -> i32 {
    let mut floors = 0;
    for c in data.chars() {
        match c {
            ')' => floors -= 1,
            '(' => floors += 1,
            _ => (),
        };
    }

    floors
}

fn part_two(data: &String) -> usize {
    let mut floors = 0;
    let mut position = 0;

    for (index, c) in data.char_indices() {
        match c {
            ')' => floors -= 1,
            '(' => floors += 1,
            _ => (),
        };

        if floors == -1 {
            position = index;
            break;
        }
    }

    position + 1
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let data = fs::read_to_string(file_path)?;

    println!("part 1: {}", part_one(&data));
    println!("part 2: {}", part_two(&data));

    Ok(())
}
