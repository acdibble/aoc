use std::time::SystemTime;
use utils::{Chart, Coord};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Open,
    Wall,
    Up,
    Right,
    Down,
    Left,
}

impl Tile {
    fn rotated_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
            _ => unreachable!(),
        }
    }

    fn rotated_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            _ => unreachable!(),
        }
    }

    fn move_coord(&self, Coord(x, y): Coord) -> Coord {
        match self {
            Self::Up => Coord(x, y - 1),
            Self::Right => Coord(x + 1, y),
            Self::Down => Coord(x, y + 1),
            Self::Left => Coord(x - 1, y),
            _ => unreachable!(),
        }
    }

    fn is_passable(&self) -> bool {
        !matches!(self, Tile::Wall | Tile::Empty)
    }

    fn value(&self) -> i64 {
        match self {
            Tile::Up => 3,
            Tile::Right => 0,
            Tile::Down => 1,
            Tile::Left => 2,
            _ => unreachable!(),
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug)]
enum Instruction {
    Right,
    Left,
    Amount(i64),
}

impl Instruction {
    fn apply(&self, tile: &Tile) -> Tile {
        match self {
            Self::Right => tile.rotated_right(),
            Self::Left => tile.rotated_left(),
            _ => unreachable!(),
        }
    }

    fn amount(&self) -> i64 {
        match self {
            Self::Amount(amount) => *amount,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            ' ' => Self::Empty,
            '.' => Self::Open,
            '#' => Self::Wall,
            _ => unreachable!(),
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Self::Empty => ' ',
            Self::Open => '.',
            Self::Wall => '#',
            Self::Up => '^',
            Self::Right => '>',
            Self::Down => 'v',
            Self::Left => '<',
        }
    }
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let mut result = Vec::new();

    let mut current = 0;

    for ch in line.chars() {
        match ch {
            'L' => {
                result.push(Instruction::Amount(current));
                result.push(Instruction::Left);
                current = 0;
            }
            'R' => {
                result.push(Instruction::Amount(current));
                result.push(Instruction::Right);
                current = 0;
            }
            '0'..='9' => {
                current *= 10;
                current += ch.to_digit(10).unwrap() as i64;
            }
            _ => unreachable!(),
        }
    }

    result.push(Instruction::Amount(current));
    result
}

fn part_one() -> i64 {
    let mut chart: Chart<Tile> = Chart::new();

    let mut lines = DATA.lines().enumerate();

    let mut start = None;

    while let Some((y, line)) = lines.next() {
        if line == "" {
            break;
        }

        for (x, ch) in line.char_indices() {
            let tile = Tile::from(ch);
            let coord = Coord(x as i64, y as i64);

            if start.is_none() && matches!(tile, Tile::Open) {
                start = Some(coord)
            }

            chart.overwrite(&coord, tile);
        }
    }

    let instructions = parse_instructions(lines.next().unwrap().1);
    let mut location = start.unwrap();
    chart.overwrite(&location, Tile::Right);
    let mut instructions = instructions.into_iter();

    while let Some(inst) = instructions.next() {
        let current = chart.get(&location).copied().unwrap_or_default();
        chart.overwrite(&location, current);
        for _ in 0..inst.amount() {
            let next_location = current.move_coord(location);
            let next_tile = chart.get(&next_location).copied().unwrap_or_default();

            if next_tile.is_passable() {
                location = next_location;
                chart.overwrite(&location, current);
                continue;
            }

            if matches!(next_tile, Tile::Wall) {
                break;
            }

            let mut new_location = match current {
                Tile::Up => Coord(location.0, chart.bottom()),
                Tile::Right => Coord(chart.left(), location.1),
                Tile::Down => Coord(location.0, chart.top()),
                Tile::Left => Coord(chart.right(), location.1),
                _ => unreachable!(),
            };

            while matches!(
                chart.get(&new_location).copied().unwrap_or_default(),
                Tile::Empty
            ) {
                new_location = current.move_coord(new_location);
            }

            let new_tile = chart.get(&new_location).unwrap();
            if !new_tile.is_passable() {
                break;
            }

            location = new_location;
            chart.overwrite(&location, current);
        }

        if let Some(inst) = instructions.next() {
            let current = inst.apply(&current);
            chart.overwrite(&location, current);
        }
    }

    1000 * (location.1 + 1) + 4 * (location.0 + 1) + chart.get(&location).unwrap().value()
}

fn part_two() -> i32 {
    0
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
