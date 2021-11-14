use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut min = i32::MAX;
        let mut max = i32::MIN;

        for value in line.split_ascii_whitespace() {
            let num = value.parse().unwrap();

            min = min.min(num);
            max = max.max(num);
        }

        sum += max - min;
    }

    sum
}

fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_ascii_whitespace()
                .map(|value| value.parse().unwrap())
                .collect();

            for (start, value1) in nums.iter().enumerate() {
                for value2 in nums.iter().skip(start + 1) {
                    if value1 % value2 == 0 {
                        return value1 / value2;
                    }

                    if value2 % value1 == 0 {
                        return value2 / value1;
                    }
                }
            }

            unreachable!()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(
                "5 1 9 5
7 5 3
2 4 6 8"
            ),
            18
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(
                "5 9 2 8
9 4 7 3
3 8 6 5"
            ),
            9
        );
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
