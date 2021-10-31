use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn decompress(line: &str) -> String {
    let mut chars = line.chars();

    let mut buffer = String::new();
    let mut output = String::new();

    let mut length_buffer = String::new();
    let mut repeat_amount_buffer = String::new();

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                loop {
                    match chars.next().unwrap() {
                        'x' => break,
                        num => length_buffer.push(num),
                    }
                }

                let length = length_buffer.parse().unwrap();

                loop {
                    match chars.next().unwrap() {
                        ')' => break,
                        num => repeat_amount_buffer.push(num),
                    }
                }

                let repeat_amount = repeat_amount_buffer.parse().unwrap();

                for _ in 0..length {
                    match chars.next() {
                        Some(c) => buffer.push(c),
                        None => panic!(),
                    }
                }

                output.push_str(buffer.repeat(repeat_amount).as_str());

                buffer.clear();
                length_buffer.clear();
                repeat_amount_buffer.clear();
            }
            _ => output.push(c),
        }
    }

    output
}

#[test]
fn test_decompress() -> std::io::Result<()> {
    assert_eq!(decompress("ADVENT"), "ADVENT");
    assert_eq!(decompress("A(1x5)BC"), "ABBBBBC");
    assert_eq!(decompress("(3x3)XYZ"), "XYZXYZXYZ");
    assert_eq!(decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
    assert_eq!(decompress("(6x1)(1x3)A"), "(1x3)A");
    assert_eq!(decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    assert_eq!(
        decompress(decompress("X(8x2)(3x3)ABCY").as_str()),
        "XABCABCABCABCABCABCY"
    );
    Ok(())
}

fn part_one(input: &String) -> usize {
    decompress(input.as_str()).len()
}

fn part_two(input: &String) -> usize {
    let mut result = input.clone();
    while result.contains("(") {
        result = decompress(result.as_str());
    }

    result.len()
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
