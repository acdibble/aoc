use std::{
    collections::BTreeMap,
    iter::{Cycle, Enumerate},
    str::Chars,
    time::SystemTime,
};
use utils::math::traits::*;

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug)]
struct Node {
    left: &'static str,
    right: &'static str,
}

fn parse_input() -> (
    Enumerate<Cycle<Chars<'static>>>,
    BTreeMap<&'static str, Node>,
) {
    let mut lines = DATA.trim().lines();

    let directions = lines.next().unwrap().chars().cycle().enumerate();

    let mut graph = BTreeMap::new();

    for line in lines.skip(1) {
        let name = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        graph.insert(name, Node { left, right });
    }

    (directions, graph)
}

fn part_one() -> i32 {
    let (directions, graph) = parse_input();

    let mut current = graph.get("AAA").unwrap();

    for (count, direction) in directions {
        let next = match direction {
            'L' => current.left,
            'R' => current.right,
            _ => unreachable!("Invalid direction"),
        };

        if next == "ZZZ" {
            return count as i32 + 1;
        }

        current = graph.get(next).unwrap();
    }

    unreachable!()
}

fn part_two() -> usize {
    let (directions, graph) = parse_input();

    let mut positions: Vec<_> = graph
        .iter()
        .filter_map(|(k, v)| if k.ends_with("A") { Some((v, 0)) } else { None })
        .collect();

    for (count, direction) in directions {
        let mut all_cycles_found = true;

        for (pos, cycle_length) in &mut positions {
            if *cycle_length == 0 {
                all_cycles_found = false;
            } else {
                continue;
            }

            let next = match direction {
                'L' => pos.left,
                'R' => pos.right,
                _ => unreachable!("Invalid direction"),
            };

            if next.ends_with("Z") && *cycle_length == 0 {
                *cycle_length = count + 1;
            }

            *pos = graph.get(next).unwrap();
        }

        if all_cycles_found {
            break;
        }
    }

    positions
        .into_iter()
        .map(|(_, cycle_length)| cycle_length)
        .reduce(|a, b| a.lcm(b))
        .unwrap()
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
