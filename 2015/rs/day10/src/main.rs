use std::env;
use std::fs;
use std::path::Path;

fn see_and_say(input: &String, iterations: usize) -> usize {
    let mut current_string = input.clone();
    let mut next_string = "".to_owned();

    for _ in 0..iterations {
        let mut count = 1;
        let mut it = current_string.chars();
        let mut current_char = it.next().unwrap();

        while let Some(c) = it.next() {
            if c == current_char {
                count += 1;
            } else {
                next_string += count.to_string().as_str();
                next_string.push(current_char);

                current_char = c;
                count = 1;
            }
        }

        next_string += count.to_string().as_str();
        next_string.push(current_char);
        current_string = next_string;
        next_string = "".to_owned();
    }

    current_string.len()
}

fn part_one(input: &String) -> usize {
    see_and_say(input, 40)
}

fn part_two(input: &String) -> usize {
    see_and_say(input, 50)
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    println!("part 1: {}", part_one(&input));
    println!("part 2: {}", part_two(&input));

    Ok(())
}
