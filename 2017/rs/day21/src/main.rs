use std::cmp::Ordering;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Pixel {
    On,
    Off,
}

impl Pixel {
    fn from_char(ch: char) -> Self {
        match ch {
            '#' => Self::On,
            '.' => Self::Off,
            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq)]
enum Form {
    One,
    Two,
    Three,
    Four,
}

impl Form {
    fn next(&self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::Three,
            Self::Three => Self::Four,
            Self::Four => Self::One,
        }
    }
}

struct Grid {
    layout: Vec<Vec<Pixel>>,
    form: Form,
}

impl Grid {
    fn merge(&mut self, grids: &Vec<&Grid>) {
        let mut square = 0;

        for i in 1.. {
            match (i * i).cmp(&grids.len()) {
                Ordering::Equal => {
                    square = i;
                    break;
                }
                Ordering::Less => continue,
                Ordering::Greater => unreachable!(),
            }
        }

        let sub_grid_len = grids[0].layout.len();
        let new_len = sub_grid_len * square;
        for row in &mut self.layout {
            row.clear();
            row.reserve(new_len - row.len())
        }

        while self.layout.len() < new_len {
            self.layout.push(Vec::with_capacity(new_len));
        }

        for (offset, slice) in grids.as_slice().chunks(square).enumerate() {
            for grid in slice {
                for i in 0..grid.layout.len() {
                    self.layout[offset * sub_grid_len + i].extend(&grid.layout[i]);
                }
            }
        }
    }

    fn from(layout: Vec<Vec<Pixel>>) -> Self {
        Self {
            layout,
            form: Form::One,
        }
    }

    fn rotate(&mut self) {
        let len = self.layout.len() - 1;

        let mut temp = Pixel::Off;

        for y in 0..len {
            for x in y..(len - y) {
                let mut to_swap = (x, y);
                for _ in 0..5 {
                    std::mem::swap(&mut temp, &mut self.layout[to_swap.1][to_swap.0]);
                    to_swap = (len - to_swap.1, to_swap.0);
                }
            }
        }
    }

    fn flip_horizontally(&mut self) {
        self.layout.reverse();
    }

    fn rearrange(&mut self) {
        match self.form {
            Form::One | Form::Two | Form::Three => self.rotate(),
            Form::Four => self.flip_horizontally(),
        }

        self.form = self.form.next()
    }

    fn count_pixels(&self, kind: Pixel) -> usize {
        self.layout
            .iter()
            .map(|row| {
                row.iter()
                    .map(|p| if *p == kind { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }

    fn tick(&mut self, patterns: &Vec<(Grid, Grid)>) {
        let len = self.layout.len();
        let size = if len % 2 == 0 { 2 } else { 3 };

        let mut sub_grids = Vec::new();

        for y in (0..len).step_by(size) {
            for x in (0..len).step_by(size) {
                let mut sub_grid = Grid::from(
                    (0..size)
                        .into_iter()
                        .map(|offset| {
                            self.layout[y + offset][x..x + size]
                                .iter()
                                .map(|p| *p)
                                .collect()
                        })
                        .collect(),
                );

                for iteration in 0..8 {
                    match patterns.iter().find(|(input, _)| input == &sub_grid) {
                        Some((_, output)) => {
                            sub_grids.push(output);
                            break;
                        }
                        _ => (),
                    }
                    if iteration == 7 {
                        unreachable!()
                    }
                    sub_grid.rearrange();
                }
            }
        }

        self.merge(&sub_grids);
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        self.layout.len() == other.layout.len()
            && self.layout.iter().zip(other.layout.iter()).all(|(a, b)| {
                a.iter()
                    .zip(b.iter())
                    .all(|(pixel_a, pixel_b)| pixel_a == pixel_b)
            })
    }
}

impl Eq for Grid {}

impl std::str::FromStr for Grid {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            form: Form::One,
            layout: string
                .split('/')
                .map(|line| line.chars().map(Pixel::from_char).collect())
                .collect(),
        })
    }
}

fn solve(input: &str, iterations: usize) -> usize {
    let mut grid: Grid = ".#./..#/###".parse().unwrap();

    let patterns: Vec<(Grid, Grid)> = input
        .lines()
        .map(|line| {
            let mut grids = line.split(" => ");

            (
                grids.next().unwrap().parse().unwrap(),
                grids.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    for _ in 0..iterations {
        grid.tick(&patterns)
    }

    grid.count_pixels(Pixel::On)
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

    time_it(|| println!("part 1: {}", solve(&input, 5)));
    time_it(|| println!("part 2: {}", solve(&input, 18)));

    Ok(())
}
