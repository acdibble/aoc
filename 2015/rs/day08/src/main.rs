use std::env;
use std::fs;
use std::path::Path;

fn part_one(input: &String) -> usize {
    let mut code_length = 0;
    let mut string_length = 0;

    for line in input.lines() {
        let len = line.len();
        code_length += line.len();
        let mut it = line[1..(len - 1)].chars();
        while let Some(c) = it.next() {
            match c {
                '\\' => {
                    if let Some('x') = it.next() {
                        it.next();
                        it.next();
                        string_length += 1;
                    } else {
                        string_length += 1;
                    }
                }
                _ => string_length += 1,
            }
        }
    }

    code_length - string_length
}

fn part_two(input: &String) -> usize {
    let mut original_length = 0;
    let mut new_length = 0;

    for line in input.lines() {
        original_length += line.len();
        new_length += 2;
        for c in line.chars() {
            if matches!(c, '\\' | '"') {
                new_length += 1;
            }
            new_length += 1;
        }
    }

    new_length - original_length
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    println!("part 1: {}", part_one(&input));
    println!("part 2: {}", part_two(&input));

    Ok(())
}
