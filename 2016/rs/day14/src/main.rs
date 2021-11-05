use crypto::digest::Digest;
use crypto::md5::Md5;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn compute_md5(string: &String) -> String {
    let mut hash = Md5::new();
    hash.input_str(string.as_str());
    hash.result_str()
}

fn remove_previous_number(string: &mut String, number: usize) {
    let mut length = 1;
    let mut current = number;
    while current % 10 != current {
        current /= 10;
        length += 1;
    }

    string.replace_range((string.len() - length).., "");
}

fn find_triple(string: &String) -> Option<char> {
    let mut chars = string.chars();

    let mut first: char;
    let mut second = chars.next().unwrap();
    let mut third = chars.next().unwrap();

    while let Some(next) = chars.next() {
        first = second;
        second = third;
        third = next;

        if first == second && second == third {
            return Some(first);
        }
    }

    None
}

fn find_quintuple(string: &String, c: char) -> bool {
    let mut chars = string.chars();

    let mut first: char;
    let mut second = chars.next().unwrap();
    let mut third = chars.next().unwrap();
    let mut fourth = chars.next().unwrap();
    let mut fifth = chars.next().unwrap();

    while let Some(next) = chars.next() {
        first = second;
        second = third;
        third = fourth;
        fourth = fifth;
        fifth = next;

        if c == first && first == second && second == third && third == fourth && fourth == fifth {
            return true;
        }
    }

    false
}

fn find_index_64(salt: &String, additional_hashings: usize) -> usize {
    let mut to_hash = salt.clone();

    let mut soughts = VecDeque::new();

    let mut key_count = 0;

    for current_index in 0.. {
        to_hash.push_str(current_index.to_string().as_str());
        let mut hash = compute_md5(&to_hash);
        for _ in 0..additional_hashings {
            hash = compute_md5(&hash);
        }

        for _ in 0..soughts.len() {
            let (c, original_index, max_index) = soughts.pop_front().unwrap();
            if current_index > max_index {
                continue;
            }

            if find_quintuple(&hash, c) {
                key_count += 1;

                if key_count == 64 {
                    return original_index;
                }
            } else {
                soughts.push_back((c, original_index, max_index))
            }
        }

        if let Some(c) = find_triple(&hash) {
            soughts.push_back((c, current_index, current_index + 1000))
        }

        remove_previous_number(&mut to_hash, current_index);
    }

    unreachable!()
}

fn part_one(salt: &String) -> usize {
    find_index_64(salt, 0)
}

fn part_two(salt: &String) -> usize {
    find_index_64(salt, 2016)
}

fn time_it<F, T>(fun: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();
    let result = fun();
    println!("Time elapsed: {} ms", start.elapsed().unwrap().as_millis());
    result
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let salt = fs::read_to_string(file_path)?;

    time_it(|| println!("part 1: {}", part_one(&salt)));
    time_it(|| println!("part 2: {}", part_two(&salt)));

    Ok(())
}
