use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(input: &str) -> u32 {
    let zipped = input.chars().zip(input.chars().cycle().skip(1));

    let mut sum = 0;

    for (current, next) in zipped {
        if current == next {
            sum += current.to_digit(10).unwrap()
        }
    }

    sum
}

fn part_two(input: &str) -> u32 {
    let half_len = input.len() / 2;

    let first_half_chars = input[..half_len].chars();
    let second_half_chars = input[half_len..].chars();
    let zipped = first_half_chars.zip(second_half_chars);

    let mut sum = 0;

    for (c1, c2) in zipped {
        if c1 == c2 {
            sum += c1.to_digit(10).unwrap() * 2;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn part_one_test() {
        assert_eq!(part_one("1122"), 3);
        assert_eq!(part_one("1111"), 4);
        assert_eq!(part_one("1234"), 0);
        assert_eq!(part_one("91212129"), 9);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two("1212"), 6);
        assert_eq!(part_two("1221"), 0);
        assert_eq!(part_two("123425"), 4);
        assert_eq!(part_two("123123"), 12);
        assert_eq!(part_two("12131415"), 4);
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
