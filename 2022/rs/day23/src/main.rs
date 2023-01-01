use std::{collections::HashMap, time::SystemTime};
use utils::{Chart, Coord, Direction};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Elf,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Elf,
            _ => unreachable!(),
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Self::Empty => '.',
            Self::Elf => '#',
        }
    }
}

fn neighbors(direction: &Direction) -> [Coord; 3] {
    match direction {
        Direction::North => [Coord(-1, 1), Coord(0, 1), Coord(1, 1)],
        Direction::East => [Coord(1, 1), Coord(1, 0), Coord(1, -1)],
        Direction::South => [Coord(-1, -1), Coord(0, -1), Coord(1, -1)],
        Direction::West => [Coord(-1, 1), Coord(-1, 0), Coord(-1, -1)],
    }
}

fn solve(max_round: Option<i32>) -> i32 {
    let mut directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]
    .iter()
    .cycle();

    let mut counts = HashMap::new();
    let mut map = Chart::new();
    let offset = DATA.lines().fold(0, |acc, _| acc + 1) / 2;

    for (y, line) in DATA.lines().enumerate() {
        for (x, ch) in line.char_indices() {
            if ch == '#' {
                map.overwrite(&Coord(x as i64, offset - y as i64), Tile::from(ch));
            }
        }
    }

    let mut stack = Vec::new();

    for round in 1..(max_round.unwrap_or(i32::MAX)) {
        for elf in &map {
            if matches!(elf.1, Tile::Empty) {
                continue;
            }

            let mut mv = None;
            let mut has_neighbor = false;

            'outer: for dir in directions.clone().take(4) {
                let new_location = elf.0.step(&dir);
                for neighbor in neighbors(&dir).map(|n| n + *elf.0) {
                    if matches!(map.get(&neighbor), Some(Tile::Elf)) {
                        has_neighbor = true;
                        continue 'outer;
                    }
                }

                mv = mv.or(Some(new_location));
            }

            if !has_neighbor {
                continue;
            }

            if let Some(mv) = mv {
                let entry = counts.entry(mv).or_insert(0);
                *entry += 1;
                stack.push((*elf.0, mv));
            }
        }

        if stack.len() == 0 {
            if max_round.is_none() {
                return round;
            }

            break;
        }

        while let Some((current, new)) = stack.pop() {
            if counts[&new] != 1 {
                continue;
            }

            map.overwrite(&current, Tile::Empty);
            map.overwrite(&new, Tile::Elf);
        }

        counts.clear();
        directions.next();
    }

    map.iter_grid()
        .fold(0, |acc, (_, tile)| acc + matches!(tile, Tile::Empty) as i32)
}

fn part_one() -> i32 {
    solve(Some(10))
}

fn part_two() -> i32 {
    solve(None)
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
