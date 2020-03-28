use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_path(args: &[&str]) -> std::path::PathBuf {
    let mut path = env::current_dir().expect("this didn't work?");
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
    let path = get_path(&["data.txt"]);
    let file = File::open(path).expect("couldn't read file");
    let reader = BufReader::new(file);
    let mut result = 0;
    for line in reader.lines() {
        let fuel: i32 = line.expect("this is a line").parse().unwrap();
        result += calculate_requirement(fuel)
    }
    println!("result: {}", result);
}
