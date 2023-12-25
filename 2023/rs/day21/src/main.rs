use std::{collections::HashSet, time::SystemTime, vec};
use utils::{Chart, Direction, Point, Translate};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug, Clone, Copy)]
enum Tile {
    Start,
    Garden,
    Rock,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Garden,
            '#' => Self::Rock,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

impl Tile {
    fn is_passable(&self) -> bool {
        !matches!(self, Self::Rock)
    }

    fn is_start(&self) -> bool {
        matches!(self, Self::Start)
    }
}

fn part_one() -> usize {
    let max_steps = 64;

    let plot = Chart::<Tile>::from(
        DATA.trim()
            .lines()
            .map(|l| l.chars().map(Tile::from).collect())
            .collect::<Vec<_>>(),
    );

    let start = plot
        .iter()
        .find_map(
            |(point, tile)| {
                if tile.is_start() {
                    Some(point)
                } else {
                    None
                }
            },
        )
        .unwrap();

    let mut current = HashSet::from([start]);
    let mut buffer = HashSet::new();

    for _ in 0..max_steps {
        buffer.clear();

        for p in current.iter() {
            for dir in Direction::all() {
                let new_p = p.translate(dir);
                match plot.get(new_p) {
                    Some(tile) if tile.is_passable() => {
                        buffer.insert(new_p);
                    }
                    _ => {}
                }
            }
        }

        std::mem::swap(&mut current, &mut buffer);
    }

    current.len()
}

fn part_two() -> usize {
    let max_steps = 1000;

    let plot = Chart::<Tile>::from(
        DATA.trim()
            .lines()
            .map(|l| l.chars().map(Tile::from).collect())
            .collect::<Vec<_>>(),
    );

    let start = plot
        .iter()
        .find_map(
            |(point, tile)| {
                if tile.is_start() {
                    Some(point)
                } else {
                    None
                }
            },
        )
        .unwrap();

    let width = plot.width() as i32;
    let height = plot.height() as i32;

    let mut current = HashSet::from([start]);
    let mut buffer = HashSet::new();

    let mut changes = vec![];

    for it in 0..max_steps {
        if (it - 65) % 131 == 0 {
            changes.push(current.len() as i32);

            if changes.len() == 3 {
                break;
            }
        }

        buffer.clear();

        for p in current.iter() {
            for dir in Direction::all() {
                let next_loc = p.translate(dir);

                let mut canonical_x = next_loc.x % width;
                if canonical_x.is_negative() {
                    canonical_x += width;
                }
                let mut canonical_y = next_loc.y % height;
                if canonical_y.is_negative() {
                    canonical_y += height;
                }

                let tile = plot.get(Point::from((canonical_x, canonical_y))).unwrap();

                if tile.is_passable() {
                    buffer.insert(next_loc);
                }
            }
        }

        std::mem::swap(&mut current, &mut buffer);
    }

    let mut first_changes = vec![];

    for window in changes.windows(2) {
        first_changes.push(window[1] - window[0]);
    }

    let mut second_changes = vec![];

    for window in first_changes.windows(2) {
        second_changes.push(window[1] - window[0]);
    }

    let target_len = (26501365 - 65) / 131 - 3;

    let mut result = *changes.last().unwrap() as usize;
    let mut change_by = *first_changes.last().unwrap() as usize;
    let diff = *second_changes.last().unwrap() as usize;

    for _ in 0..=target_len {
        change_by += diff;
        result += change_by;
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
