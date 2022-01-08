use std::env;
use std::fs;
use std::path::Path;

const GRID_SIZE: usize = 100;
const NEIGHBORS: [(i32, i32); 8] = [
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Dark,
    Light,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dark
    }
}

struct Grid([[Cell; GRID_SIZE]; GRID_SIZE]);

impl Grid {
    fn count_lights(&self) -> i32 {
        let mut total = 0;

        for row in &self.0 {
            for c in row {
                if *c == Cell::Light {
                    total += 1;
                }
            }
        }

        total
    }

    fn get_neighbor(&self, x: usize, y: usize, neighbor: &(i32, i32)) -> Option<Cell> {
        let neighbor_y = if neighbor.1.is_negative() {
            y.checked_sub(neighbor.1.wrapping_abs() as usize)?
        } else {
            y + neighbor.1 as usize
        };
        let neighbor_x = if neighbor.0.is_negative() {
            x.checked_sub(neighbor.0.wrapping_abs() as usize)?
        } else {
            x + neighbor.0 as usize
        };
        let row = self.0.get(neighbor_y)?;
        let cell = row.get(neighbor_x)?;
        Some(*cell)
    }

    fn cycle(self) -> Self {
        let mut storage = [[Cell::Dark; GRID_SIZE]; GRID_SIZE];

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let light_count = NEIGHBORS.iter().fold(0, |acc, neighbor| {
                    match self.get_neighbor(x, y, neighbor) {
                        Some(Cell::Light) => acc + 1,
                        _ => acc,
                    }
                });

                storage[y][x] = match self.0[y][x] {
                    Cell::Light => match light_count {
                        2 | 3 => Cell::Light,
                        _ => Cell::Dark,
                    },
                    Cell::Dark => match light_count {
                        3 => Cell::Light,
                        _ => Cell::Dark,
                    },
                }
            }
        }

        Self(storage)
    }

    fn from(string: &String) -> Self {
        let mut storage = [[Cell::Dark; GRID_SIZE]; GRID_SIZE];

        for (y, line) in string.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                storage[y][x] = match c {
                    '.' => Cell::Dark,
                    '#' => Cell::Light,
                    _ => unreachable!(),
                }
            }
        }

        Self(storage)
    }

    fn light_corners(&mut self) {
        self.0[0][0] = Cell::Light;
        self.0[GRID_SIZE - 1][0] = Cell::Light;
        self.0[0][GRID_SIZE - 1] = Cell::Light;
        self.0[GRID_SIZE - 1][GRID_SIZE - 1] = Cell::Light;
    }
}

impl Grid {
    #[cfg(debug_assertions)]

    fn print(&self) {
        for row in &self.0 {
            for c in row {
                print!("{}", if *c == Cell::Light { '#' } else { '.' });
            }
            print!("\n");
        }
    }
}

fn part_one(grid: Grid) -> i32 {
    let mut current = grid;

    for _ in 0..100 {
        {
            #[cfg(debug_assertions)]
            current.print();
        }
        current = current.cycle();
        {
            #[cfg(debug_assertions)]
            println!();
        }
    }

    {
        #[cfg(debug_assertions)]
        current.print();
    }

    current.count_lights()
}

fn part_two(grid: Grid) -> i32 {
    let mut current = grid;

    for _ in 0..100 {
        current.light_corners();
        {
            #[cfg(debug_assertions)]
            current.print();
        }
        current = current.cycle();
        {
            #[cfg(debug_assertions)]
            println!();
        }
    }

    current.light_corners();
    {
        #[cfg(debug_assertions)]
        current.print();
    }

    current.count_lights()
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;
    println!("part 1: {}", part_one(Grid::from(&input)));
    println!("part 2: {}", part_two(Grid::from(&input)));

    Ok(())
}
