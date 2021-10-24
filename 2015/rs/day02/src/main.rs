use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Package(i32, i32, i32);

fn parse_line(line: &str) -> Package {
    let mut dimensions = [0; 3];

    for (index, dim) in line.split('x').enumerate() {
        dimensions[index] = dim.parse().expect("failed to parse number");
    }

    dimensions.sort();

    Package(dimensions[0], dimensions[1], dimensions[2])
}

fn part_one(data: &Vec<Package>) -> i32 {
    let mut total_area = 0;

    for Package(l, w, h) in data {
        total_area += l * w * 2 + l * h * 2 + w * h * 2 + l * w;
    }

    total_area
}

fn part_two(data: &Vec<Package>) -> i32 {
    let mut total_area = 0;

    for Package(l, w, h) in data {
        total_area += l * 2 + w * 2 + l * w * h;
    }

    total_area
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let packages: Vec<Package> = fs::read_to_string(file_path)?
        .lines()
        .map(parse_line)
        .collect();

    println!("part 1: {}", part_one(&packages));
    println!("part 2: {}", part_two(&packages));

    Ok(())
}
