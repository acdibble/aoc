use std::{collections::HashSet, time::SystemTime};
use utils::{Coord, Direction, Map};

const DATA: &'static str = include_str!("../data.txt");

fn parse_map() -> Map<char> {
    Map::from(
        DATA.trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>(),
    )
}

fn flood_fill(map: &Map<char>, visited: &mut HashSet<Coord>, coord: Coord, expected: char) -> i32 {
    let mut stack = vec![coord];

    let mut perimeter = 0;
    let mut area = 0;

    while let Some(coord) = stack.pop() {
        if !visited.insert(coord) {
            continue;
        }

        area += 1;

        for dir in Direction::all() {
            let next = coord.translate(dir);

            let neighbor = map.get(next).copied().unwrap_or_default();

            if neighbor == expected {
                stack.push(next);
            } else {
                perimeter += 1;
            }
        }
    }

    area * perimeter
}

fn solve<F>(fun: F) -> i32
where
    F: Fn(&Map<char>, &mut HashSet<Coord>, Coord, char) -> i32,
{
    let map = parse_map();
    let mut visited = HashSet::new();

    map.iter_coords()
        .flat_map(|c| {
            (!visited.contains(&c))
                .then(|| fun(&map, &mut visited, c, map.get(c).copied().unwrap()))
        })
        .sum()
}

fn part_one() -> i32 {
    solve(&flood_fill)
}

fn walk_perimeter(map: &Map<char>, visited: &mut HashSet<Coord>, start: Coord, ch: char) -> i32 {
    let mut stack = vec![start];

    let mut area = 0;

    let mut edges = HashSet::new();

    while let Some(coord) = stack.pop() {
        if !visited.insert(coord) {
            continue;
        }

        area += 1;

        for dir in Direction::all() {
            let next = coord.translate(dir);

            let neighbor = map.get(next).copied().unwrap_or_default();

            if neighbor == ch {
                stack.push(next);
            } else {
                edges.insert(coord);
            }
        }
    }

    let mut sides = 0;

    while edges.len() != 0 {
        let start = edges.iter().next().copied().unwrap();

        let start_dir = Direction::all()
            .into_iter()
            .find(|d| {
                let next = start.translate(*d);
                map.get(next).copied().unwrap_or_default() != ch
            })
            .unwrap()
            .turn_left();

        let mut current_loc = start;
        let mut current_dir = start_dir;
        let mut started = false;

        while !started || !(current_loc == start && current_dir == start_dir) {
            started = true;
            edges.remove(&current_loc);
            let ahead = current_loc.translate(current_dir);
            if map.get(ahead).copied().unwrap_or_default() != ch {
                current_dir = current_dir.turn_left();
                sides += 1;

                continue;
            }

            let to_right = map
                .get(ahead.translate(current_dir.turn_right()))
                .copied()
                .unwrap_or_default();

            if to_right == ch {
                current_dir = current_dir.turn_right();
                current_loc = ahead.translate(current_dir);
                sides += 1;
            } else {
                current_loc = ahead;
            }
        }
    }

    area * sides
}

fn part_two() -> i32 {
    solve(&walk_perimeter)
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
