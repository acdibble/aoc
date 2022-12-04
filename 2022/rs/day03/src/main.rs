use std::{collections::HashSet, hash::Hash, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

fn find_common_char(line: &str) -> char {
    let middle = line.len() / 2;

    let first_pocket = line[0..middle].chars().collect::<HashSet<_>>();

    for ch in line[middle..].chars() {
        if first_pocket.contains(&ch) {
            return ch;
        }
    }

    unreachable!();
}

fn get_priority(ch: char) -> i32 {
    match ch {
        'a'..='z' => 1 + ch as i32 - 'a' as i32,
        'A'..='Z' => 27 + ch as i32 - 'A' as i32,
        _ => unreachable!(),
    }
}

fn part_one() -> i32 {
    let mut result = 0;

    for line in DATA.lines() {
        let ch = find_common_char(line);
        result += get_priority(ch);
    }

    result
}

fn part_two() -> i32 {
    let mut result = 0;

    let mut lines = DATA.lines();

    while let Some(first) = lines.next() {
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();

        let first_set = first.chars().collect::<HashSet<_>>();
        let second_set = second.chars().collect::<HashSet<_>>();
        let third_set = third.chars().collect::<HashSet<_>>();

        let intersection = first_set
            .intersection(&second_set)
            .into_iter()
            .cloned()
            .collect::<HashSet<_>>();

        let ch = intersection
            .intersection(&third_set)
            .into_iter()
            .next()
            .unwrap();

        result += get_priority(*ch);
    }

    result
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

fn main() {
    time_it(|| println!("part 1: {}", part_one()));
    time_it(|| println!("part 2: {}", part_two()));
}
