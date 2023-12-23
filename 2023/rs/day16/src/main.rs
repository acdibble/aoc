use std::{
    collections::{BTreeSet, VecDeque},
    iter,
    time::SystemTime,
};
use utils::{Direction, Point, Translate};

const DATA: &'static str = include_str!("../data.txt");

enum Tile {
    Mirror,     // '/'
    Backmirror, // '\'
    HSplitter,  // '-'
    VSplitter,  // '|'
    Empty,      // '.'
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '/' => Self::Mirror,
            '\\' => Self::Backmirror,
            '-' => Self::HSplitter,
            '|' => Self::VSplitter,
            '.' => Self::Empty,
            _ => unreachable!("{value}"),
        }
    }
}

impl Tile {
    fn traverse(&self, from: Direction) -> impl Iterator<Item = Direction> {
        match self {
            Self::Empty => iter::once(Some(from.rev())).chain(iter::once(None)),
            Self::HSplitter => match from {
                Direction::East | Direction::West => {
                    iter::once(Some(from.rev())).chain(iter::once(None))
                }
                Direction::North | Direction::South => {
                    iter::once(Some(Direction::East)).chain(iter::once(Some(Direction::West)))
                }
            },
            Self::VSplitter => match from {
                Direction::North | Direction::South => {
                    iter::once(Some(from.rev())).chain(iter::once(None))
                }
                Direction::East | Direction::West => {
                    iter::once(Some(Direction::North)).chain(iter::once(Some(Direction::South)))
                }
            },
            Self::Mirror => match from {
                Direction::East => iter::once(Some(Direction::South)).chain(iter::once(None)),
                Direction::South => iter::once(Some(Direction::East)).chain(iter::once(None)),
                Direction::West => iter::once(Some(Direction::North)).chain(iter::once(None)),
                Direction::North => iter::once(Some(Direction::West)).chain(iter::once(None)),
            },
            Self::Backmirror => match from {
                Direction::East => iter::once(Some(Direction::North)).chain(iter::once(None)),
                Direction::North => iter::once(Some(Direction::East)).chain(iter::once(None)),
                Direction::West => iter::once(Some(Direction::South)).chain(iter::once(None)),
                Direction::South => iter::once(Some(Direction::West)).chain(iter::once(None)),
            },
        }
        .flat_map(|v| v)
    }
}

fn count_points(cavern: &[Vec<Tile>], start: (Point, Direction)) -> usize {
    let x_bounds = 0..cavern[0].len() as i32;
    let y_bounds = 0..cavern.len() as i32;

    let mut queue = VecDeque::from([start]);
    let mut visited_states = BTreeSet::from([start]);
    let mut visited_points = BTreeSet::from([start.0]);

    while let Some((loc, direction)) = queue.pop_front() {
        let next = loc.translate(direction);

        if !x_bounds.contains(&next.x) || !y_bounds.contains(&next.y) {
            continue;
        }

        for dir in cavern[next.y as usize][next.x as usize].traverse(direction.rev()) {
            if visited_states.insert((next, dir)) {
                visited_points.insert(next);
                queue.push_back((next, dir));
            }
        }
    }

    visited_points.len()
}

fn part_one() -> usize {
    let cavern: Vec<Vec<_>> = DATA
        .trim()
        .lines()
        .map(|l| l.chars().map(Tile::from).collect())
        .collect();

    count_points(&cavern, (Point::from((0, 0)), Direction::East))
}

fn part_two() -> usize {
    let cavern: Vec<Vec<_>> = DATA
        .trim()
        .lines()
        .map(|l| l.chars().map(Tile::from).collect())
        .collect();

    let max_x = cavern[0].len() - 1;
    let max_y = cavern.len() - 1;

    (0..=max_x)
        .flat_map(|x| [(x, 0, Direction::South), (x, max_y, Direction::North)])
        .chain((0..=max_y).flat_map(|y| [(0, y, Direction::East), (max_x, y, Direction::West)]))
        .map(|(x, y, dir)| count_points(&cavern, (Point::from((x, y)), dir)))
        .max()
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
