use std::fs::File;
use std::io::Read;

fn main() {
    let mut polymers = String::new();

    File::open("day05/input.txt")
        .unwrap()
        .read_to_string(&mut polymers)
        .unwrap();

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

    println!("{}", shortest);
}
