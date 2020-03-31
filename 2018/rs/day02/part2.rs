use std::fs::File;
use std::io::{BufReader, BufRead};

fn get_common_chars(a: &String, b: &String) -> String {
    a.chars().zip(b.chars()).filter_map(|(a, b)| {
        match a == b {
            true => Some(a),
            false => None,
        }
    })
    .collect()
}

fn only_one_difference(a: &String, b: &String) -> bool {
    let mut diff_found = false;
    for (char_a, char_b) in a.chars().zip(b.chars()) {
        if char_a != char_b {
            if diff_found {
                return false;
            }
            diff_found = true;
        }
    }

    return true;
}

fn main() {
    let ids: Vec<String> = BufReader::new(File::open("day02/input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect();

    for (i, id) in ids.iter().enumerate() {
        for j in (i + 1)..ids.len() {
            if only_one_difference(id, &ids[j]) {
                let common_chars = get_common_chars(id, &ids[j]);
                println!("{}", common_chars);
                return;
            }
        }
    }
}
