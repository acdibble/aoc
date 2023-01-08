use std::time::SystemTime;
use utils::{Chart, Coord};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    fn apply(&self, tile: &Tile) -> Option<Tile> {
        match self {
            Self::Right => Some(tile.rotated_right()),
            Self::Left => Some(tile.rotated_left()),
            _ => None,
        }
    }

    fn amount(&self) -> Option<i64> {
        match self {
            Self::Amount(amount) => Some(*amount),
            _ => None,
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
            _ => unreachable!(),
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

fn find_new_location_2d(
    chart: &Chart<Tile>,
    location: &Coord,
    current: &Tile,
) -> Option<(Coord, Tile)> {
    let mut new_location = current.move_coord(*location);
    match chart.get(&new_location).copied().unwrap_or_default() {
        Tile::Empty => {
            new_location = match current {
                Tile::Up => Coord(location.0, chart.bottom()),
                Tile::Right => Coord(chart.left(), location.1),
                Tile::Down => Coord(location.0, chart.top()),
                Tile::Left => Coord(chart.right(), location.1),
                _ => unreachable!(),
            };
        }
        Tile::Wall => return None,
        _ => {}
    }

    while matches!(
        chart.get(&new_location).copied().unwrap_or_default(),
        Tile::Empty
    ) {
        new_location = current.move_coord(new_location);
    }

    if !chart.get(&new_location).unwrap().is_passable() {
        return None;
    }

    Some((new_location, *current))
}

fn run<F>(find_new_location: F) -> i64
where
    F: Fn(&Chart<Tile>, &Coord, &Tile) -> Option<(Coord, Tile)>,
{
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
    let mut instructions = instructions.into_iter();

    let mut current = Tile::Right;

    while let Some(inst) = instructions.next() {
        if let Some(new_tile) = inst.apply(&current) {
            current = new_tile;
            continue;
        }

        for _ in 0..inst.amount().unwrap() {
            if let Some(result) = find_new_location(&chart, &location, &current) {
                location = result.0;
                current = result.1;
            } else {
                break;
            }
        }
    }

    1000 * (location.1 + 1) + 4 * (location.0 + 1) + current.value()
}

fn part_one() -> i64 {
    run(&find_new_location_2d)
}

fn find_new_location_3d(
    chart: &Chart<Tile>,
    location: &Coord,
    current: &Tile,
) -> Option<(Coord, Tile)> {
    let (new_location, new_direction) = match (location, current) {
        (Coord(149, y @ 0..=49), &Tile::Right) => (Coord(99, 49 - y + 100), Tile::Left),
        (Coord(x @ 100..=149, 49), &Tile::Down) => (Coord(99, x - 50), Tile::Left),
        (Coord(99, y @ 50..=99), &Tile::Right) => (Coord(y + 50, 49), Tile::Up),
        (Coord(99, y @ 100..=149), &Tile::Right) => (Coord(149, 149 - y), Tile::Left),
        (Coord(x @ 50..=99, 149), &Tile::Down) => (Coord(49, x + 100), Tile::Left),
        (Coord(49, y @ 150..=199), &Tile::Right) => (Coord(y - 100, 149), Tile::Up),
        (Coord(x @ 0..=49, 199), &Tile::Down) => (Coord(x + 100, 0), Tile::Down),
        (Coord(0, y @ 150..=199), &Tile::Left) => (Coord(y - 100, 0), Tile::Down),
        (Coord(0, y @ 100..=149), &Tile::Left) => (Coord(50, 149 - y), Tile::Right),
        (Coord(x @ 0..=49, 100), &Tile::Up) => (Coord(50, x + 50), Tile::Right),
        (Coord(50, y @ 50..=99), &Tile::Left) => (Coord(y - 50, 100), Tile::Down),
        (Coord(50, y @ 0..=49), &Tile::Left) => (Coord(0, 49 - y + 100), Tile::Right),
        (Coord(x @ 50..=99, 0), &Tile::Up) => (Coord(0, x + 100), Tile::Right),
        (Coord(x @ 100..=149, 0), &Tile::Up) => (Coord(x - 100, 199), Tile::Up),
        _ => (current.move_coord(*location), *current),
    };

    match chart.get(&new_location) {
        Some(Tile::Open) => Some((new_location, new_direction)),
        Some(Tile::Wall) => None,
        _ => unreachable!(),
    }
}

fn part_two() -> i64 {
    run(&find_new_location_3d)
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
