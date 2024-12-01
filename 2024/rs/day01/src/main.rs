use std::{collections::HashMap, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

fn parse_lists() -> (Vec<i32>, Vec<i32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in DATA.lines() {
        let mut parts = line.split_whitespace();
        list1.push(parts.next().unwrap().parse().unwrap());
        list2.push(parts.next().unwrap().parse().unwrap());
    }
    (list1, list2)
}

fn part_one() -> i32 {
    let (mut a, mut b) = parse_lists();

    a.sort();
    b.sort();

    a.into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part_two() -> i32 {
    let (a, b) = parse_lists();

    let counts = b.into_iter().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|count| *count += 1).or_insert(1);
        map
    });

    a.into_iter()
        .map(|val| counts.get(&val).unwrap_or(&0) * val)
        .sum()
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
