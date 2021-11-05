use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn generate_data(input: &str, length: usize) -> String {
    let mut result = input.to_string();

    while result.len() < length {
        let mut buffer = String::with_capacity(result.len() * 2 + 1);
        buffer.push_str(&result);
        buffer.push('0');

        let b = result.chars().rev().map(|c| match c {
            '0' => '1',
            _ => '0',
        });

        buffer.extend(b);
        result = buffer;
    }

    result.replace_range(length.., "");

    result
}

#[test]
fn test_generate_data() {
    assert_eq!(generate_data("1", 3), "100");
    assert_eq!(generate_data("0", 3), "001");
    assert_eq!(generate_data("11111", 11), "11111000000");
    assert_eq!(
        generate_data("111100001010", 25),
        "1111000010100101011110000"
    );
    assert_eq!(generate_data("10000", 20), "10000011110010000111");
}

fn find_checksum(input: &str, length: usize) -> String {
    let mut current = generate_data(input, length);

    while current.len() % 2 == 0 {
        current = current
            .as_bytes()
            .chunks(2)
            .map(|chunk| match chunk {
                [b'1', b'1'] | [b'0', b'0'] => '1',
                _ => '0',
            })
            .collect();
    }

    current
}

#[test]
fn test_find_checksum() {
    assert_eq!(find_checksum("110010110100", 12), "100");
    assert_eq!(find_checksum("10000011110010000111", 20), "01100");
}

fn time_it<F, T>(fun: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();
    let result = fun();
    println!("Time elapsed: {} ms", start.elapsed().unwrap().as_millis());
    result
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("part 1: {}", find_checksum(input.as_str(), 272)));
    time_it(|| println!("part 2: {}", find_checksum(input.as_str(), 35651584)));

    Ok(())
}
