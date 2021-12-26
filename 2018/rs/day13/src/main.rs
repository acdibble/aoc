use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Copy, Clone)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&self) -> Self {
        match self {
            Self::Left => Self::Straight,
            Self::Straight => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Vertical,
    Horizontal,
    Slash,
    Backslash,
    Intersection,
    Collision,
    Cart(Direction, Turn),
}

impl Tile {
    fn from_char(ch: char) -> Self {
        match ch {
            ' ' => Self::Empty,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            '/' => Self::Slash,
            '\\' => Self::Backslash,
            '+' => Self::Intersection,
            '^' => Self::Cart(Direction::Up, Turn::Left),
            '>' => Self::Cart(Direction::Right, Turn::Left),
            'v' => Self::Cart(Direction::Down, Turn::Left),
            '<' => Self::Cart(Direction::Left, Turn::Left),
            _ => unreachable!(),
        }
    }

    fn char(&self) -> char {
        match self {
            Self::Empty => ' ',
            Self::Vertical => '|',
            Self::Horizontal => '-',
            Self::Slash => '/',
            Self::Backslash => '\\',
            Self::Intersection => '+',
            Self::Collision => 'X',
            Self::Cart(Direction::Up, _) => '^',
            Self::Cart(Direction::Right, _) => '>',
            Self::Cart(Direction::Down, _) => 'v',
            Self::Cart(Direction::Left, _) => '<',
        }
    }
}

#[derive(Clone, Debug)]
struct Mine(Vec<Vec<Tile>>);

impl Mine {
    fn from_str(string: &str) -> Self {
        Self(
            string
                .lines()
                .map(|l| l.chars().map(Tile::from_char).collect())
                .collect(),
        )
    }

    fn clone_blank(&self) -> Self {
        Self(
            self.0
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|tile| match tile {
                            Tile::Cart(Direction::Up, _) | Tile::Cart(Direction::Down, _) => {
                                Tile::Vertical
                            }
                            Tile::Cart(Direction::Right, _) | Tile::Cart(Direction::Left, _) => {
                                Tile::Horizontal
                            }
                            _ => *tile,
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn iter(&self) -> impl Iterator<Item = ((usize, usize), &Tile)> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, tile)| ((x, y), tile)))
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut Tile)> {
        self.0.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, tile)| ((x, y), tile))
        })
    }

    fn get(&self, x: usize, y: usize) -> &Tile {
        if let Some(row) = self.0.get(y) {
            if let Some(tile) = row.get(x) {
                return tile;
            }
        }

        unreachable!()
    }

    fn set(&mut self, x: usize, y: usize, new_tile: Tile) {
        if let Some(row) = self.0.get_mut(y) {
            if let Some(tile) = row.get_mut(x) {
                *tile = new_tile;
                return;
            }
        }

        unreachable!()
    }
}

fn simulate(input: &str, first_crash: bool) -> (usize, usize) {
    let mut map = Mine::from_str(input);

    let blank_map = map.clone_blank();

    let mut buffer = blank_map.clone();

    loop {
        for ((x, y), tile) in map.iter() {
            match tile {
                Tile::Cart(direction, turn) => {
                    let (next_x, next_y) = match direction {
                        Direction::Up => (x, y - 1),
                        Direction::Right => (x + 1, y),
                        Direction::Down => (x, y + 1),
                        Direction::Left => (x - 1, y),
                    };

                    let buffer_current = buffer.get(x, y);

                    if matches!(buffer_current, Tile::Cart(_, _) | Tile::Collision) {
                        if first_crash {
                            return (next_x, next_y);
                        }

                        buffer.set(x, y, Tile::Collision);
                        continue;
                    }

                    let buffer_next = buffer.get(next_x, next_y);
                    if matches!(buffer_next, Tile::Cart(_, _) | Tile::Collision) {
                        if first_crash {
                            return (next_x, next_y);
                        }

                        buffer.set(next_x, next_y, Tile::Collision);
                        continue;
                    }

                    match buffer_next {
                        Tile::Intersection => {
                            let new_direction = match turn {
                                Turn::Left => direction.left(),
                                Turn::Right => direction.right(),
                                _ => *direction,
                            };
                            let new_turn = turn.next();
                            buffer.set(next_x, next_y, Tile::Cart(new_direction, new_turn));
                        }
                        Tile::Slash => {
                            let new_direction = match direction {
                                Direction::Right | Direction::Left => direction.left(),
                                Direction::Up | Direction::Down => direction.right(),
                            };
                            buffer.set(next_x, next_y, Tile::Cart(new_direction, *turn));
                        }
                        Tile::Backslash => {
                            let new_direction = match direction {
                                Direction::Right | Direction::Left => direction.right(),
                                Direction::Up | Direction::Down => direction.left(),
                            };
                            buffer.set(next_x, next_y, Tile::Cart(new_direction, *turn));
                        }
                        Tile::Vertical | Tile::Horizontal => buffer.set(next_x, next_y, *tile),
                        Tile::Collision | Tile::Empty | Tile::Cart(_, _) => {
                            unreachable!(buffer_next.char())
                        }
                    }
                }
                _ => {}
            }
        }

        for ((x, y), tile) in buffer.iter_mut() {
            if matches!(*tile, Tile::Collision) {
                *tile = *blank_map.get(x, y);
            }
        }

        std::mem::swap(&mut buffer, &mut map);

        let mut cart_count = 0;
        let mut cart_location = (0, 0);

        for ((x, y), tile) in buffer.iter_mut() {
            if matches!(*tile, Tile::Cart(_, _)) {
                *tile = *blank_map.get(x, y);
                cart_count += 1;
                cart_location = (x, y);
            }
        }

        debug_assert!(cart_count > 2 || cart_count == 1);

        if cart_count == 1 {
            return cart_location;
        }
    }
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

    time_it(|| println!("part 1: {:?}", simulate(&input, true)));
    time_it(|| println!("part 2: {:?}", simulate(&input, false)));

    Ok(())
}
