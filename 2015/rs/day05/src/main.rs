use std::env;
use std::fs;
use std::path::Path;

fn part_one_is_nice(line: &str) -> bool {
    let mut vowel_count = 0;
    let mut has_double = false;
    let mut it = line.chars().peekable();
    while let Some(c) = it.next() {
        if let Some(next) = it.peek() {
            if c == *next {
                has_double = true;
            }
        }
        match c {
            'a' => {
                vowel_count += 1;
                if matches!(it.peek(), Some(&'b')) {
                    return false;
                }
            }
            'c' => {
                if matches!(it.peek(), Some(&'d')) {
                    return false;
                }
            }
            'p' => {
                if matches!(it.peek(), Some(&'q')) {
                    return false;
                }
            }
            'x' => {
                if matches!(it.peek(), Some(&'y')) {
                    return false;
                }
            }
            'e' | 'i' | 'o' | 'u' => vowel_count += 1,
            _ => (),
        }
    }

    has_double && vowel_count >= 3
}

fn part_one(input: &String) -> i32 {
    let mut nice_count = 0;

    for line in input.lines() {
        nice_count += part_one_is_nice(line) as i32;
    }

    nice_count
}

fn part_two_is_nice(line: &str) -> bool {
    let mut pairs = Vec::<(char, char, usize)>::new();

    let mut has_double_double = false;
    let mut has_split_pair = false;

    let mut previous: char;
    let mut current = '\0';
    let mut next: char;

    let mut it = line.chars().enumerate().peekable();

    while let Some((index, c)) = it.next() {
        previous = current;
        current = c;
        if let Some((_, next_c)) = it.peek() {
            next = *next_c;

            if !has_double_double {
                for (a, b, loc) in &pairs {
                    if *a == current && *b == next && index - loc != 1 {
                        has_double_double = true;
                    }
                }

                pairs.push((current, next, index));
            }
        } else {
            next = '\0';
        }

        if previous == next {
            has_split_pair = true;
        }
    }

    has_double_double && has_split_pair
}

fn part_two(input: &String) -> i32 {
    let mut nice_count = 0;

    for line in input.lines() {
        nice_count += part_two_is_nice(line) as i32;
    }

    nice_count
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    println!("part 1: {}", part_one(&input));
    println!("part 2: {}", part_two(&input));

    Ok(())
}
