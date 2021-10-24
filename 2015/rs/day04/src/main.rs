use md5;
use std::env;
use std::fs;
use std::path::Path;

fn mine(input: &String, sought: &str) -> i32 {
    let mut result = 0;

    loop {
        let string = format!("{}{}", input, result);
        let hash = format!("{:x}", md5::compute(string.as_bytes()));
        if hash.starts_with(sought) {
            break;
        }
        result += 1;
    }

    result
}

fn part_one(input: &String) -> i32 {
    mine(input, "00000")
}

fn part_two(input: &String) -> i32 {
    mine(input, "000000")
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    println!("part 1: {}", part_one(&input));
    println!("part 2: {}", part_two(&input));

    Ok(())
}
