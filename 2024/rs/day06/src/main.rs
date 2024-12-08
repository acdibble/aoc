use std::{collections::HashSet, fmt::Debug, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(i32, i32);

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    fn translate(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self(self.0, self.1 - 1),
            Direction::Down => Self(self.0, self.1 + 1),
            Direction::Left => Self(self.0 - 1, self.1),
            Direction::Right => Self(self.0 + 1, self.1),
        }
    }
}

#[derive(Debug, Clone)]

struct Map<T: Debug + Clone + Copy> {
    tiles: Vec<Vec<T>>,
}

impl<T: Debug + Clone + Copy> Map<T> {
    fn from(tiles: Vec<Vec<T>>) -> Self {
        Self { tiles }
    }

    fn get(&self, coord: Coord) -> Option<T> {
        if coord.0 < 0 || coord.1 < 0 {
            return None;
        }

        self.tiles
            .get(coord.1 as usize)
            .and_then(|row| row.get(coord.0 as usize).copied())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Guard(Direction),
    Obstacle,
    Open,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Obstacle,
            '.' => Self::Open,
            '^' => Self::Guard(Direction::Up),
            '>' => Self::Guard(Direction::Right),
            'v' => Self::Guard(Direction::Down),
            '<' => Self::Guard(Direction::Left),
            _ => unreachable!(),
        }
    }
}

fn parse_input() -> (Map<Tile>, Coord, Direction) {
    let mut start = Coord::new(0, 0);
    let mut direction = Direction::Up;

    let lines = DATA
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, ch)| match Tile::from(ch) {
                    Tile::Guard(dir) => {
                        start = Coord::new(x as i32, y as i32);
                        direction = dir;

                        Tile::Open
                    }
                    tile => tile,
                })
                .collect()
        })
        .collect();

    (Map::from(lines), start, direction)
}

fn walk<F>(
    map: &Map<Tile>,
    mut current: Coord,
    mut direction: Direction,
    mut fun: F,
) -> Result<(), ()>
where
    F: FnMut(Coord, Direction) -> bool,
{
    while let Some(tile) = map.get(current.translate(direction)) {
        match tile {
            Tile::Obstacle => {
                direction = direction.turn_right();
            }
            Tile::Open => {
                current = current.translate(direction);
                if !fun(current, direction) {
                    return Err(());
                }
            }
            Tile::Guard(_) => unreachable!(),
        }
    }

    Ok(())
}

fn part_one() -> i32 {
    let (map, current, direction) = parse_input();
    let mut seen = HashSet::from([current]);

    walk(&map, current, direction, |coord, _| {
        seen.insert(coord);
        true
    })
    .unwrap();

    seen.len() as i32
}

fn part_two() -> i32 {
    let (mut map, current, direction) = parse_input();
    let mut seen = HashSet::from([(current, direction)]);
    let mut result = 0;

    for y in 0..map.tiles.len() {
        for x in 0..map.tiles[0].len() {
            if map.tiles[y][x] == Tile::Obstacle {
                continue;
            }

            seen.clear();

            map.tiles[y][x] = Tile::Obstacle;

            match walk(&map, current, direction, |coord, dir| {
                seen.insert((coord, dir))
            }) {
                Ok(_) => {}
                Err(_) => result += 1,
            }

            map.tiles[y][x] = Tile::Open;
        }
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
