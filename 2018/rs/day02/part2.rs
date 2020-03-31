use std::fs::File;
use std::io::{BufReader, BufRead};

fn get_common_chars(a: &String, b: &String) -> Vec<char> {
    a.chars().zip(b.chars()).filter_map(|(a, b)| {
        match a == b {
            true => Some(a),
            false => None,
        }
    })
    .collect()
}

fn main() {
    let ids: Vec<String> = BufReader::new(File::open("day02/input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect();

    for (i, id) in ids.iter().enumerate() {
        for j in i..ids.len() {
            let common_chars = get_common_chars(id, &ids[j]);
            if common_chars.len() == id.len() - 1 {
                println!("{}", common_chars.into_iter().collect::<String>());
                return;
            }
        }
    }
}
