use std::{
    collections::{BinaryHeap, HashSet},
    time::SystemTime,
};
use utils::{lcm, Chart, Coord};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, coord: &Coord) -> Coord {
        match self {
            Direction::Up => *coord + Coord(0, -1),
            Direction::Down => *coord + Coord(0, 1),
            Direction::Left => *coord + Coord(-1, 0),
            Direction::Right => *coord + Coord(1, 0),
        }
    }

    fn from_u8(value: u8) -> impl Iterator<Item = Self> {
        [
            if (value & 0b0001) == Self::Up.into() {
                Some(Self::Up)
            } else {
                None
            },
            if (value & 0b0010) == Self::Down.into() {
                Some(Self::Down)
            } else {
                None
            },
            if (value & 0b0100) == Self::Left.into() {
                Some(Self::Left)
            } else {
                None
            },
            if (value & 0b1000) == Self::Right.into() {
                Some(Self::Right)
            } else {
                None
            },
        ]
        .into_iter()
        .flat_map(|v| v)
    }
}

impl Into<u8> for Direction {
    fn into(self) -> u8 {
        match self {
            Self::Up => 0b0001,
            Self::Down => 0b0010,
            Self::Left => 0b0100,
            Self::Right => 0b1000,
        }
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0b0001 => Self::Up,
            0b0010 => Self::Down,
            0b0100 => Self::Left,
            0b1000 => Self::Right,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!(),
        }
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Empty,
    Wall,
    Blizzard(u8),
}

impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}

impl std::ops::AddAssign<Direction> for Tile {
    fn add_assign(&mut self, dir: Direction) {
        match self {
            Tile::Empty => *self = Tile::Blizzard(dir.into()),
            Tile::Blizzard(value) => *value |= Into::<u8>::into(dir),
            _ => unreachable!(),
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Self::Empty => '.',
            Self::Wall => '#',
            Self::Blizzard(value) => match value.count_ones() {
                4 => '4',
                3 => '3',
                2 => '2',
                1 => Direction::from(value).into(),
                _ => unreachable!("{value}"),
            },
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            _ => Self::Blizzard(Direction::from(value).into()),
        }
    }
}

struct State {
    position: Coord,
    minutes: usize,
    distance: i64,
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.distance.cmp(&self.distance) {
            std::cmp::Ordering::Equal => other.minutes.cmp(&self.minutes),
            ordering => ordering,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_charts() -> Vec<Chart<Tile>> {
    let mut chart = Chart::new();

    for (y, line) in DATA.lines().skip(1).enumerate() {
        let line = &line[1..line.len() - 1];
        for (x, ch) in line.char_indices() {
            if ch == '#' {
                break;
            }
            chart.overwrite(&Coord(x as i64, y as i64), Tile::from(ch));
        }
    }

    let len = lcm(chart.bottom() + 1, chart.right() + 1) - 1;
    let mut charts = Vec::with_capacity(len as usize);
    charts.push(chart);
    let mut stack = vec![];

    for _ in 0..len {
        let mut chart = charts.last().unwrap().clone();
        for (coord, tile) in &mut chart {
            if !matches!(*tile, Tile::Empty) {
                stack.push((*coord, *tile));
            }
            *tile = Tile::Empty;
        }

        while let Some((coord, tile)) = stack.pop() {
            match tile {
                Tile::Empty => {}
                Tile::Blizzard(value) => {
                    for dir in Direction::from_u8(value) {
                        let new_coord = dir.apply(&coord);
                        let chart_tile = if let Some(chart_tile) = chart.get_mut(&new_coord) {
                            chart_tile
                        } else {
                            let new_coord = match dir {
                                Direction::Up => Coord(new_coord.0, chart.bottom()),
                                Direction::Down => Coord(new_coord.0, chart.top()),
                                Direction::Left => Coord(chart.right(), new_coord.1),
                                Direction::Right => Coord(chart.left(), new_coord.1),
                            };
                            chart.get_mut(&new_coord).unwrap()
                        };
                        *chart_tile += dir;
                    }
                }
                _ => unreachable!(),
            }
        }

        charts.push(chart);
    }

    charts
}

fn do_trip(round_trip: bool) -> usize {
    let charts = parse_charts();
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    let mut total = 0;
    for (start, goal) in [
        (
            Coord(0, -1),
            Coord(charts[0].right(), charts[0].bottom() + 1),
        ),
        (
            Coord(charts[0].right(), charts[0].bottom() + 1),
            Coord(0, -1),
        ),
        (
            Coord(0, -1),
            Coord(charts[0].right(), charts[0].bottom() + 1),
        ),
    ]
    .into_iter()
    .take(if round_trip { 3 } else { 1 })
    {
        seen.clear();
        heap.clear();
        heap.push(State {
            position: start,
            minutes: total,
            distance: goal.distance_to(&Coord(0, 0)),
        });
        let mut min = usize::MAX;

        while let Some(state) = heap.pop() {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)] {
                let new_position = state.position + Coord(dx, dy);
                let minutes = state.minutes + 1;
                if minutes + new_position.distance_to(&goal) as usize > min {
                    continue;
                }

                if new_position == goal {
                    min = min.min(minutes);
                    continue;
                }

                let chart = &charts[minutes % charts.len()];

                if !matches!(chart.get(&new_position), Some(Tile::Empty)) && new_position != start {
                    continue;
                }

                if seen.insert((new_position, minutes)) {
                    heap.push(State {
                        minutes,
                        position: new_position,
                        distance: new_position.distance_to(&goal),
                    });
                }
            }
        }

        total += min - total;
    }

    total
}

fn part_one() -> usize {
    do_trip(false)
}

fn part_two() -> usize {
    do_trip(true)
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
