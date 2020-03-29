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

fn calculate_requirement(reader: BufReader<File>) -> i32 {
    let mut result = 0;
    for line in reader.lines() {
        let fuel: i32 = line.unwrap().parse().unwrap();
        result += fuel / 3 - 2;
    }
    return result;
}

fn main() {
    let path = get_path(&["day01", "data.txt"]);
    let file = File::open(path).unwrap();
    let result = calculate_requirement(BufReader::new(file));
    println!("result: {}", result);
}
