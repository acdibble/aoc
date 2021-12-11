use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn count_occurrences(map: &mut HashMap<char, i32>, id: &str) -> (i32, i32) {
    for ch in id.chars() {
        let counter = map.entry(ch).or_insert(0);
        *counter += 1;
    }

    let mut result = (0, 0);

    for v in map.values() {
        match v {
            2 => result.0 = 1,
            3 => result.1 = 1,
            _ => (),
        }
    }

    return result;
}

fn part_one(input: &str) -> i32 {
    let mut map = HashMap::<char, i32>::new();
    let (double, triple): (i32, i32) = input.lines().fold((0, 0), |(d, t), line| {
        map.clear();
        let result = count_occurrences(&mut map, line);
        (d + result.0, t + result.1)
    });

    double * triple
}

fn get_common_chars(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter_map(|(a, b)| match a == b {
            true => Some(a),
            false => None,
        })
        .collect()
}

fn only_one_difference(a: &str, b: &str) -> bool {
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

fn part_two(input: &str) -> String {
    let ids: Vec<_> = input.lines().collect();

    for (i, id) in ids.iter().enumerate() {
        for id2 in ids.iter().skip(i + 1) {
            if only_one_difference(id, id2) {
                return get_common_chars(id, id2);
            }
        }
    }

    unreachable!()
}

fn time_it<F, T>(fun: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();
    let result = fun();
    println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros());
    result
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("part 1: {}", part_one(&input)));
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}
