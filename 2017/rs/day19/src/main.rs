use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Pipe,
    Dash,
    Cross,
    Letter(char),
    Wall,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }
}

#[derive(Debug)]
struct Diagram {
    layout: Vec<Vec<Tile>>,
    direction: Direction,
    coord: (usize, usize),
    current_tile: Tile,
    letters: String,
    steps: i32,
}

impl Diagram {
    fn new(input: &str) -> Self {
        let layout: Vec<Vec<Tile>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '|' => Tile::Pipe,
                        '-' => Tile::Dash,
                        '+' => Tile::Cross,
                        'A'..='Z' => Tile::Letter(c),
                        _ => Tile::Wall,
                    })
                    .collect()
            })
            .collect();

        let x = layout[0].iter().position(|t| *t == Tile::Pipe).unwrap();

        Self {
            layout,
            direction: Direction::Down,
            coord: (x, 0),
            current_tile: Tile::Pipe,
            letters: String::new(),
            steps: 0,
        }
    }

    fn next_coord(&self) -> (usize, usize) {
        let (x, y) = self.coord;

        match self.direction {
            Direction::Up => (x, y.saturating_sub(1)),
            Direction::Down => (x, y + 1),
            Direction::Left => (x.saturating_sub(1), y),
            Direction::Right => (x + 1, y),
        }
    }

    fn turn(&mut self) -> (usize, usize) {
        let current = self.direction;
        self.direction = current.left();
        let (x, y) = self.next_coord();

        match self.layout.get(y) {
            Some(row) => match row.get(x) {
                Some(Tile::Wall) => (),
                Some(_) => return (x, y),
                None => (),
            },
            _ => (),
        }

        self.direction = current.right();
        let (x, y) = self.next_coord();

        match self.layout.get(y) {
            Some(row) => match row.get(x) {
                Some(Tile::Wall) => (),
                Some(_) => return (x, y),
                None => (),
            },
            _ => (),
        }

        unreachable!()
    }

    fn tick(&mut self) -> Option<()> {
        self.steps += 1;
        self.coord = match self.current_tile {
            Tile::Pipe | Tile::Dash => self.next_coord(),
            Tile::Letter(c) => {
                self.letters.push(c);
                self.next_coord()
            }
            Tile::Cross => self.turn(),
            _ => unreachable!(),
        };

        let (x, y) = self.coord;
        self.current_tile = match self.layout.get(y) {
            Some(row) => match row.get(x) {
                Some(tile @ (Tile::Cross | Tile::Dash | Tile::Letter(_) | Tile::Pipe)) => *tile,
                Some(Tile::Wall) => {
                    if matches!(self.current_tile, Tile::Letter(_)) {
                        return None;
                    }

                    unreachable!()
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        Some(())
    }
}

fn solve(input: &str) -> (String, i32) {
    let mut diagram = Diagram::new(input);

    while diagram.tick().is_some() {}

    (diagram.letters, diagram.steps)
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

    time_it(|| println!("part (1, 2): {:?}", solve(&input)));

    Ok(())
}
