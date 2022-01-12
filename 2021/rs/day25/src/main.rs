use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Copy, Clone, Debug)]
enum Tile {
    Empty,
    East,
    South,
}

impl Tile {
    fn is_empty(&self) -> bool {
        matches!(self, Tile::Empty)
    }
}

impl std::convert::TryFrom<char> for Tile {
    type Error = char;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'v' => Ok(Self::South),
            '>' => Ok(Self::East),
            '.' => Ok(Self::Empty),
            _ => Err(ch),
        }
    }
}

struct SeaFloor {
    map: Vec<Vec<Tile>>,
    y_dimension: usize,
    x_dimension: usize,
}

impl SeaFloor {
    fn from_str(string: &str) -> Self {
        let map: Vec<Vec<_>> = string
            .lines()
            .map(|l| l.chars().flat_map(Tile::try_from).collect())
            .collect();
        let y_dimension = map.len();
        let x_dimension = map.first().unwrap().len();

        Self {
            map,
            y_dimension,
            x_dimension,
        }
    }

    fn next_y(&self, y: usize) -> usize {
        (y + 1) % self.y_dimension
    }

    fn next_x(&self, x: usize) -> usize {
        (x + 1) % self.x_dimension
    }

    fn get_east(&self, x: usize, y: usize) -> &Tile {
        if let Some(row) = self.map.get(y) {
            if let Some(tile) = row.get(self.next_x(x)) {
                return tile;
            }
        }

        unreachable!()
    }

    fn get_south(&self, x: usize, y: usize) -> &Tile {
        if let Some(row) = self.map.get(self.next_y(y)) {
            if let Some(tile) = row.get(x) {
                return tile;
            }
        }

        unreachable!()
    }

    fn insert(&mut self, x: usize, y: usize, tile: Tile) {
        if let Some(row) = self.map.get_mut(y) {
            if let Some(loc) = row.get_mut(x) {
                debug_assert!(matches!(*loc, Tile::Empty));
                *loc = tile;
                return;
            }
        }

        unreachable!()
    }

    fn insert_east(&mut self, x: usize, y: usize, tile: Tile) {
        let x = self.next_x(x);
        if let Some(row) = self.map.get_mut(y) {
            if let Some(loc) = row.get_mut(x) {
                debug_assert!(matches!(*loc, Tile::Empty));
                *loc = tile;
                return;
            }
        }

        unreachable!()
    }

    fn insert_south(&mut self, x: usize, y: usize, tile: Tile) {
        let y = self.next_y(y);
        if let Some(row) = self.map.get_mut(y) {
            if let Some(loc) = row.get_mut(x) {
                debug_assert!(matches!(*loc, Tile::Empty));
                *loc = tile;
                return;
            }
        }

        unreachable!()
    }

    fn iter(&self) -> impl Iterator<Item = ((usize, usize), &Tile)> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, tile)| ((x, y), tile)))
    }

    fn clone_empty(&self) -> Self {
        Self {
            map: vec![vec![Tile::Empty; self.x_dimension]; self.y_dimension],
            y_dimension: self.y_dimension,
            x_dimension: self.x_dimension,
        }
    }

    fn clear(&mut self) {
        for row in &mut self.map {
            for tile in row {
                *tile = Tile::Empty;
            }
        }
    }
}

fn part_one(input: &str) -> i32 {
    let mut sea_floor = SeaFloor::from_str(input);

    let mut should_continue = true;

    let mut temp = sea_floor.clone_empty();
    let mut rounds = 0;

    while should_continue {
        rounds += 1;
        should_continue = false;

        for ((x, y), tile) in sea_floor.iter() {
            match tile {
                Tile::East => {
                    if sea_floor.get_east(x, y).is_empty() {
                        temp.insert_east(x, y, *tile);
                        should_continue = true;
                    } else {
                        temp.insert(x, y, *tile);
                    }
                }
                _ => {}
            }
        }

        for ((x, y), tile) in sea_floor.iter() {
            match tile {
                Tile::South => match (sea_floor.get_south(x, y), temp.get_south(x, y)) {
                    (Tile::East | Tile::Empty, Tile::Empty) => {
                        temp.insert_south(x, y, *tile);
                        should_continue = true;
                    }
                    _ => {
                        temp.insert(x, y, *tile);
                    }
                },
                _ => {}
            }
        }

        sea_floor.clear();
        std::mem::swap(&mut sea_floor, &mut temp);
    }

    rounds
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

    time_it(|| println!("part 1: {}", part_one(&input)));

    Ok(())
}
