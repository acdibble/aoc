use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(polymers: &str) -> usize {
    let mut output: Vec<char> = Vec::with_capacity(polymers.len());

    let min_maj_diff: i32 = 'a' as i32 - 'A' as i32;

    for ch in polymers.chars() {
        output.push(ch);

        let length = output.len();
        if length < 2 {
            continue;
        }

        let diff = (output[length - 1] as i32 - output[length - 2] as i32).abs();
        if diff == min_maj_diff {
            output.pop();
            output.pop();
        }
    }

    output.len()
}

fn part_two(polymers: &str) -> usize {
    let min_maj_diff: i32 = (b'a' - b'A') as i32;

    let tuples: Vec<(char, char)> = (b'a'..=b'z')
        .map(|b| (char::from(b), char::from(b - 32)))
        .collect();

    let mut shortest = polymers.len();
    let mut output: Vec<char> = Vec::with_capacity(polymers.len());

    for (lower, upper) in tuples {
        for ch in polymers.chars() {
            if ch == upper || ch == lower {
                continue;
            }

            output.push(ch);

            let length = output.len();
            if length < 2 {
                continue;
            }

            let diff = (output[length - 1] as i32 - output[length - 2] as i32).abs();
            if diff == min_maj_diff {
                output.pop();
                output.pop();
            }
        }

        shortest = if shortest > output.len() {
            output.len()
        } else {
            shortest
        };

        output.clear();
    }

    shortest
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
