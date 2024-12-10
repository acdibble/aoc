use std::{collections::HashSet, time::SystemTime};
use utils::{Coord, Direction, Map};

const DATA: &'static str = include_str!("../data.txt");

fn parse_input() -> (Map<i32>, Vec<Coord>) {
    let mut zeros = Vec::new();

    let map = Map::from(
        DATA.trim()
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.char_indices()
                    .map(|(x, c)| {
                        let value = c.to_digit(10).unwrap() as i32;
                        if value == 0 {
                            zeros.push(Coord::new(x as i32, y as i32));
                        }
                        value
                    })
                    .collect()
            })
            .collect::<Vec<_>>(),
    );

    (map, zeros)
}

fn walk(map: &Map<i32>, loc: Coord, tracker: &mut Tracker) {
    let current = map.get(loc).copied().unwrap();

    for dir in Direction::all() {
        let next_loc = loc.translate(dir);

        match map.get(next_loc).copied() {
            None => {}
            Some(n) if current + 1 != n => {}
            Some(9) => {
                tracker.add(next_loc);
            }
            Some(_) => {
                walk(map, next_loc, tracker);
            }
        }
    }
}

enum Tracker {
    Vec(Vec<Coord>),
    Set(HashSet<Coord>),
}

impl Tracker {
    fn add(&mut self, value: Coord) {
        match self {
            Self::Vec(v) => {
                v.push(value);
            }
            Self::Set(s) => {
                s.insert(value);
            }
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Vec(v) => v.len(),
            Self::Set(s) => s.len(),
        }
    }

    fn clear(&mut self) {
        match self {
            Self::Vec(v) => v.clear(),
            Self::Set(s) => s.clear(),
        }
    }
}

fn solve(mut tracker: Tracker) -> i32 {
    let (map, zeros) = parse_input();
    zeros
        .iter()
        .map(|&loc| {
            tracker.clear();
            walk(&map, loc, &mut tracker);
            tracker.len() as i32
        })
        .sum()
}

fn part_one() -> i32 {
    solve(Tracker::Set(HashSet::new()))
}

fn part_two() -> i32 {
    solve(Tracker::Vec(Vec::new()))
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
