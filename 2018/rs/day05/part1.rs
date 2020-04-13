use std::fs::File;
use std::io::Read;

fn main() {
    let mut polymers = String::new();

    File::open("day05/input.txt")
        .unwrap()
        .read_to_string(&mut polymers)
        .unwrap();

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

    println!("{}", output.len());
}
