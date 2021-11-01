use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn calculate_decompressed_length(input: &str, recurse: bool) -> usize {
    let mut len = 0;

    let mut chars = input.char_indices();

    let mut length_buffer = String::new();
    let mut repeat_amount_buffer = String::new();

    while let Some((_, c)) = chars.next() {
        match c {
            '(' => {
                let length = loop {
                    match chars.next().unwrap().1 {
                        'x' => break length_buffer.parse().unwrap(),
                        num => length_buffer.push(num),
                    }
                };

                let (start_index, repeat_amount) = loop {
                    match chars.next().unwrap() {
                        (i, ')') => break (i + 1, repeat_amount_buffer.parse::<usize>().unwrap()),
                        (_, num) => repeat_amount_buffer.push(num),
                    }
                };

                for _ in 0..length {
                    chars.next();
                }

                if recurse {
                    len += calculate_decompressed_length(
                        &input[start_index..start_index + length],
                        true,
                    ) * repeat_amount;
                } else {
                    len += length * repeat_amount
                }

                length_buffer.clear();
                repeat_amount_buffer.clear();
            }
            _ => len += 1,
        }
    }

    len
}

#[test]
fn test_calculate_decompressed_length() -> std::io::Result<()> {
    assert_eq!(calculate_decompressed_length("ADVENT", true), 6);
    assert_eq!(calculate_decompressed_length("A(1x5)BC", true), 7);
    assert_eq!(calculate_decompressed_length("(3x3)XYZ", true), 9);
    assert_eq!(calculate_decompressed_length("A(2x2)BCD(2x2)EFG", true), 11);
    assert_eq!(calculate_decompressed_length("(6x1)(1x3)A", true), 3);
    assert_eq!(calculate_decompressed_length("X(8x2)(3x3)ABCY", true), 20);
    assert_eq!(calculate_decompressed_length("X(8x2)(3x3)ABCY", false), 18);
    Ok(())
}

fn part_one(input: &String) -> usize {
    calculate_decompressed_length(input.as_str(), false)
}

fn part_two(input: &String) -> usize {
    calculate_decompressed_length(input.as_str(), true)
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
