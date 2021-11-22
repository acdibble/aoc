use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn right(&self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
        }
    }

    fn left(&self) -> Self {
        match self {
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Up => Self::Left,
        }
    }

    fn reverse(&self) -> Self {
        self.right().right()
    }
}

#[derive(Clone, Copy, Debug)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl TryFrom<char> for State {
    type Error = char;

    fn try_from(
        ch: char,
    ) -> std::result::Result<Self, <Self as std::convert::TryFrom<char>>::Error> {
        match ch {
            '.' => Ok(Self::Clean),
            '#' => Ok(Self::Infected),
            _ => Err(ch),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Clean
    }
}

impl State {
    fn next(&self) -> Self {
        match self {
            Self::Flagged => Self::Clean,
            Self::Clean => Self::Weakened,
            Self::Weakened => Self::Infected,
            Self::Infected => Self::Flagged,
        }
    }
}

struct Grid(HashMap<(i32, i32), State>);

impl std::str::FromStr for Grid {
    type Err = String;

    fn from_str(string: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        Ok(Grid(
            string
                .lines()
                .rev()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.chars()
                        .enumerate()
                        .map(move |(x, ch)| ((x as i32, y as i32), State::try_from(ch).unwrap()))
                })
                .collect(),
        ))
    }
}

#[derive(Debug)]
struct Carrier {
    direction: Direction,
    location: (i32, i32),
    infections_caused: i32,
}

impl Carrier {
    fn new(location: (i32, i32)) -> Self {
        Self {
            direction: Direction::Up,
            infections_caused: 0,
            location,
        }
    }
}

impl Carrier {
    fn tick(&mut self, grid: &mut Grid) {
        let entry = grid.0.entry(self.location).or_default();
        match *entry {
            State::Clean => self.direction = self.direction.left(),
            State::Weakened => (),
            State::Infected => self.direction = self.direction.right(),
            State::Flagged => self.direction = self.direction.reverse(),
        };
        *entry = entry.next();

        if matches!(*entry, State::Infected) {
            self.infections_caused += 1;
        }

        let (x, y) = self.location;
        self.location = match self.direction {
            Direction::Up => (x, y + 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y - 1),
            Direction::Left => (x - 1, y),
        };
    }
}

fn part_two(input: &str) -> i32 {
    let mut grid: Grid = input.parse().unwrap();

    let mut middle = 0;

    while grid.0.contains_key(&(middle, middle)) {
        middle += 1;
    }

    let mut carrier = Carrier::new((middle / 2, middle / 2));

    for _ in 0..10000000 {
        carrier.tick(&mut grid);
    }

    carrier.infections_caused
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

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}
