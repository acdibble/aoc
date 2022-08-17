use std::{collections::HashMap, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

fn part_one() -> i32 {
    let mut graph = HashMap::<&str, Vec<&str>>::with_capacity(1500);

    for line in DATA.trim().lines() {
        let mut split = line.split(')');
        let object = split.next().unwrap();
        let satellite = split.next().unwrap();

        let entry = graph.entry(object).or_insert_with(|| Vec::with_capacity(2));
        entry.push(satellite);
    }

    let mut stack = Vec::with_capacity(500);
    stack.push(("COM", 0));

    let mut count = 0;

    while let Some((current, depth)) = stack.pop() {
        count += depth;
        if let Some(children) = graph.get(current) {
            for child in children {
                stack.push((child, depth + 1));
            }
        }
    }

    count
}

fn part_two() -> i32 {
    let mut orbit_map = HashMap::<&str, &str>::with_capacity(2000);

    for line in DATA.trim().lines() {
        let mut split = line.split(')');
        let object = split.next().unwrap();
        let satellite = split.next().unwrap();

        orbit_map.insert(satellite, object);
    }

    let mut you_path = HashMap::<&str, i32>::new();

    let mut current = orbit_map["YOU"];
    let mut transfers = 0;

    while current != "COM" {
        you_path.insert(current, transfers);

        current = orbit_map[current];
        transfers += 1;
    }

    let mut current = orbit_map["SAN"];
    let mut transfers = 0;

    while you_path.get(current).is_none() {
        transfers += 1;
        current = orbit_map[current];
    }

    you_path[current] + transfers
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
