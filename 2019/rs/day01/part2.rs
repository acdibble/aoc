use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_path(args: &[&str]) -> std::path::PathBuf {
    let mut path = env::current_dir().unwrap();
    for arg in args.iter() {
        path.push(arg);
    }
    return path;
}

fn calculate_requirement(fuel: i32) -> i32 {
    match fuel / 3 - 2 {
        n if n <= 0 => 0,
        n => n + calculate_requirement(n),
    }
}

fn main() {
    let path = get_path(&["day01", "data.txt"]);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut result = 0;
    for line in reader.lines() {
        let fuel: i32 = line.unwrap().parse().unwrap();
        result += calculate_requirement(fuel)
    }
    println!("result: {}", result);
}
