use std::time::SystemTime;
use utils::{Coord, Direction, Map};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Box,
    Robot,
    BoxLeft,
    BoxRight,
}

impl From<Tile> for char {
    fn from(t: Tile) -> Self {
        match t {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Box => 'O',
            Tile::Robot => '@',
            Tile::BoxLeft => '[',
            Tile::BoxRight => ']',
        }
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = (*self).into();
        write!(f, "{}", c)
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            '[' => Self::BoxLeft,
            ']' => Self::BoxRight,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

fn parse_data<F>(fun: F) -> (Map<Tile>, Vec<Direction>)
where
    F: Fn(&str) -> Vec<Tile>,
{
    let mut parts = DATA.trim().split("\n\n");

    let map = Map::from(parts.next().unwrap().lines().map(fun).collect::<Vec<_>>());

    let moves = parts
        .next()
        .unwrap()
        .chars()
        .filter(|ch| *ch != '\n')
        .map(|ch| match ch {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid move: {:?}", ch),
        })
        .collect();

    (map, moves)
}

fn step(map: &mut Map<Tile>, loc: Coord, dir: Direction, swap: bool) -> bool {
    let next = loc + dir;

    match map.get(next).copied().unwrap() {
        Tile::Empty => {
            if swap {
                map.swap(loc, next);
            }
            true
        }
        Tile::BoxLeft | Tile::BoxRight if matches!(dir, Direction::Left | Direction::Right) => {
            if step(map, next, dir, swap) {
                if swap {
                    map.swap(loc, next);
                }
                true
            } else {
                false
            }
        }
        tile @ (Tile::BoxLeft | Tile::BoxRight) => {
            let pair = next
                + if tile == Tile::BoxLeft {
                    Direction::Right
                } else {
                    Direction::Left
                };
            if step(map, next, dir, swap) && step(map, pair, dir, swap) {
                if swap {
                    map.swap(loc, next);
                }
                true
            } else {
                false
            }
        }
        Tile::Box => {
            if step(map, next, dir, swap) {
                if swap {
                    map.swap(loc, next);
                }
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn part_one() -> i32 {
    let (mut map, moves) = parse_data(|line| line.chars().map(Tile::from).collect::<Vec<_>>());

    let mut loc = map.iter().find(|(_, &tile)| tile == Tile::Robot).unwrap().0;

    for dir in moves {
        if step(&mut map, loc, dir, true) {
            loc = loc + dir;
        }
    }

    map.iter()
        .flat_map(|(coord, &tile)| (tile == Tile::Box).then(|| coord.y * 100 + coord.x))
        .sum()
}

fn part_two() -> i32 {
    let (mut map, moves) = parse_data(|line| {
        line.chars()
            .flat_map(|ch| match ch {
                '.' => ['.', '.'],
                '#' => ['#', '#'],
                'O' => ['[', ']'],
                '@' => ['@', '.'],
                _ => panic!("Invalid tile: {}", ch),
            })
            .map(Tile::from)
            .collect::<Vec<_>>()
    });

    let mut loc = map.iter().find(|(_, &tile)| tile == Tile::Robot).unwrap().0;

    for dir in moves {
        if step(&mut map, loc, dir, false) && step(&mut map, loc, dir, true) {
            loc = loc + dir;
        }
    }

    map.iter()
        .flat_map(|(coord, &tile)| (tile == Tile::BoxLeft).then(|| coord.y * 100 + coord.x))
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
