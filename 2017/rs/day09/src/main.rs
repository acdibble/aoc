use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn parse_garbage(chars: &mut std::str::Chars) {
    while let Some(c) = chars.next() {
        match c {
            '>' => break,
            '!' => {
                chars.next();
            }
            _ => continue,
        }
    }
}

fn parse_group(chars: &mut std::str::Chars, depth: i32) -> i32 {
    let mut score = 0;

    while let Some(c) = chars.next() {
        match c {
            '{' => score += parse_group(chars, depth + 1),
            '<' => parse_garbage(chars),
            '}' => break,
            _ => continue,
        }
    }

    depth + score
}

fn part_one(input: &str) -> i32 {
    let mut chars = input.chars();
    chars.next();

    parse_group(&mut chars, 1)
}

fn count_patch(chars: &mut std::str::Chars) -> i32 {
    let mut count = 0;

    while let Some(c) = chars.next() {
        match c {
            '>' => break,
            '!' => {
                chars.next();
            }
            _ => count += 1,
        }
    }

    count
}

fn count_garbage(chars: &mut std::str::Chars) -> i32 {
    let mut score = 0;

    while let Some(c) = chars.next() {
        match c {
            '{' => score += count_garbage(chars),
            '<' => score += count_patch(chars),
            '}' => break,
            _ => continue,
        }
    }

    score
}

fn part_two(input: &str) -> i32 {
    let mut chars = input.chars();

    count_garbage(&mut chars)
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("{}"), 1);
        assert_eq!(part_one("{{{}}}"), 6);
        assert_eq!(part_one("{{},{}}"), 5);
        assert_eq!(part_one("{{{},{},{{}}}}"), 16);
        assert_eq!(part_one("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(part_one("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(part_one("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(part_one("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("{<>}"), 0);
        assert_eq!(part_two("{<random characters>}"), 17);
        assert_eq!(part_two("{<<<<>}"), 3);
    }
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
