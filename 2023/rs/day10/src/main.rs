use std::{
    collections::{BTreeMap, VecDeque},
    fmt::{Display, Write},
    time::SystemTime,
};
use utils::Point;

const DATA: &'static str = include_str!("../data.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rev(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    Clear,
    Ground,
    Vertical,
    Horizontal,
    NorthEastConn,
    NorthWestConn,
    SouthEastConn,
    SouthWestConn,
    Start,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEastConn,
            'J' => Tile::NorthWestConn,
            '7' => Tile::SouthWestConn,
            'F' => Tile::SouthEastConn,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => unreachable!(),
        }
    }
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        match value {
            Tile::Clear => ' ',
            Tile::Vertical => '│',
            Tile::Horizontal => '─',
            Tile::NorthEastConn => '└',
            Tile::NorthWestConn => '┘',
            Tile::SouthWestConn => '┐',
            Tile::SouthEastConn => '┌',
            Tile::Ground => '.',
            Tile::Start => 'S',
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

impl Tile {
    fn is_accessible_from(&self, dir: Direction) -> bool {
        use Direction::*;

        match (self, dir) {
            (Self::Vertical, North | South) => true,
            (Self::Horizontal, East | West) => true,
            (Self::NorthEastConn, North | East) => true,
            (Self::NorthWestConn, North | West) => true,
            (Self::SouthEastConn, South | East) => true,
            (Self::SouthWestConn, South | West) => true,
            _ => false,
        }
    }

    fn can_move_to(&self, dir: Direction) -> bool {
        use Direction::*;

        match (self, dir) {
            (Self::Start, _) => true,
            (Self::Vertical, North | South) => true,
            (Self::Horizontal, East | West) => true,
            (Self::NorthEastConn, North | East) => true,
            (Self::NorthWestConn, North | West) => true,
            (Self::SouthEastConn, South | East) => true,
            (Self::SouthWestConn, South | West) => true,
            _ => false,
        }
    }

    fn transit(&self, from: Direction) -> Direction {
        use Direction::*;
        match (self, from) {
            (Self::Vertical, North | South) => from,
            (Self::Horizontal, East | West) => from,
            (Self::NorthEastConn, South) => East,
            (Self::NorthEastConn, West) => North,
            (Self::Start | Self::NorthWestConn, South) => West,
            (Self::Start | Self::NorthWestConn, East) => North,
            (Self::SouthEastConn, North) => East,
            (Self::SouthEastConn, West) => South,
            (Self::SouthWestConn, North) => West,
            (Self::SouthWestConn, East) => South,
            _ => unreachable!("failed to transit: {} {:?}", self, from),
        }
    }
}

fn parse_chart() -> (BTreeMap<Point, Tile>, BTreeMap<Point, usize>) {
    let mut chart = BTreeMap::new();
    let mut start = None;

    for (y, line) in DATA.trim().lines().enumerate() {
        for (x, ch) in line.char_indices() {
            let tile = Tile::from(ch);
            let point = Point::from((x, y));
            if tile == Tile::Start {
                start = Some(point);
            }
            chart.insert(point, tile);
        }
    }

    let start = start.expect("failed to find start");
    let mut distances = BTreeMap::from([(start, 0usize)]);
    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((p, step_count)) = queue.pop_front() {
        let current = chart.get(&p).expect("failed to get current tile");

        for (next, dir) in [
            (p.translate_up(), Direction::South),
            (p.translate_down(), Direction::North),
            (p.translate_left(), Direction::East),
            (p.translate_right(), Direction::West),
        ] {
            if !current.can_move_to(dir.rev()) {
                continue;
            }

            if let Some(neighbor) = chart.get(&next) {
                if !neighbor.is_accessible_from(dir) {
                    continue;
                }

                let current_shortest = distances.get(&next).copied().unwrap_or(usize::MAX);

                if current_shortest > step_count + 1 {
                    distances.insert(next, step_count + 1);
                    queue.push_back((next, step_count + 1))
                }
            }
        }
    }

    (chart, distances)
}

fn part_one() -> usize {
    let (_, distances) = parse_chart();

    distances.values().max().copied().unwrap()
}

struct Surfer {
    location: Point,
    heading: Direction,
}

impl Surfer {
    fn advance(&mut self, chart: &BTreeMap<Point, Tile>) -> Box<dyn Iterator<Item = Point>> {
        // find next location in loop
        self.location = match self.heading {
            Direction::North => self.location.translate_up(),
            Direction::East => self.location.translate_right(),
            Direction::South => self.location.translate_down(),
            Direction::West => self.location.translate_left(),
        };

        let tile = chart.get(&self.location).unwrap();
        // determine our new heading based off the new tile shape
        let next_heading = tile.transit(self.heading);

        let first = self.get_point_to_right();

        // if we don't turn, we only have visited one tile to the right
        if next_heading == self.heading {
            Box::from(std::iter::once(first))
        } else {
            self.heading = next_heading;

            // otherwise we have to yield the tile before and after the turn
            Box::from(std::iter::once(first).chain(std::iter::once(self.get_point_to_right())))
        }
    }

    fn get_point_to_right(&self) -> Point {
        match self.heading {
            Direction::North => self.location.translate_right(),
            Direction::East => self.location.translate_down(),
            Direction::South => self.location.translate_left(),
            Direction::West => self.location.translate_up(),
        }
    }
}

fn flood_fill(chart: &mut BTreeMap<Point, Tile>, p: Point) {
    assert_eq!(chart.insert(p, Tile::Clear), Some(Tile::Ground));

    let mut queue = VecDeque::from([p]);

    while let Some(p) = queue.pop_front() {
        for next in [
            p.translate_down(),
            p.translate_left(),
            p.translate_right(),
            p.translate_up(),
        ] {
            if matches!(chart.get(&next), Some(Tile::Ground)) {
                queue.push_back(next);
                chart.insert(next, Tile::Clear);
            }
        }
    }
}

fn part_two() -> usize {
    let (mut chart, distances) = parse_chart();

    // replace any tiles that aren't part of the loop with ground
    for (k, v) in &mut chart {
        if !distances.contains_key(k) {
            *v = Tile::Ground;
        }
    }

    let mut starting_point = Point { x: 0, y: 0 };

    // find the loop by walking south
    while !distances.contains_key(&starting_point) {
        starting_point = starting_point.translate_down();
    }

    let mut surfer = Surfer {
        location: starting_point,
        heading: Direction::South,
    };

    // prime the surfer, drop the box it returns
    drop(surfer.advance(&chart));

    // surf the entire loop
    while surfer.location != starting_point {
        for p in surfer.advance(&chart) {
            // if we find any ground tiles to the right of the surfer
            if let Some(t) = chart.get(&p) {
                if *t == Tile::Ground {
                    // clear those ground tiles w/ flood-fill algorithm
                    flood_fill(&mut chart, p);
                }
            }
        }
    }

    // any ground remaining must have been within the loop
    chart.values().filter(|t| **t == Tile::Ground).count()
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
