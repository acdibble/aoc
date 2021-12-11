use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

const GRID_DIMENSION: usize = 10;

type Grid = [[Octopus; GRID_DIMENSION]; GRID_DIMENSION];

#[derive(Clone, Copy)]
enum Octopus {
    Normal(u32),
    Flashing,
}

fn noop(n: usize) -> Option<usize> {
    Some(n)
}

fn inc(n: usize) -> Option<usize> {
    match n + 1 {
        GRID_DIMENSION => None,
        _ => Some(n + 1),
    }
}

fn dec(n: usize) -> Option<usize> {
    match n {
        0 => None,
        _ => Some(n - 1),
    }
}

type NeighborFn = fn(usize) -> Option<usize>;

const NEIGHBORS: [(NeighborFn, NeighborFn); 8] = [
    (inc, noop),
    (inc, inc),
    (inc, dec),
    (noop, inc),
    (noop, dec),
    (dec, inc),
    (dec, noop),
    (dec, dec),
];

fn propagate_flashes(grid: &mut Grid, x: usize, y: usize, increment: bool) {
    unsafe {
        let row = grid.get_unchecked_mut(y);
        let octopus = row.get_unchecked_mut(x);
        match octopus {
            Octopus::Normal(9) if increment => *octopus = Octopus::Flashing,
            Octopus::Normal(v @ 0..=8) if increment => {
                *v += 1;
                return;
            }
            Octopus::Normal(0..=9) | Octopus::Flashing => return,
            _ => *octopus = Octopus::Flashing,
        }
    }

    for (x_fn, y_fn) in NEIGHBORS {
        if let (Some(x), Some(y)) = (x_fn(x), y_fn(y)) {
            propagate_flashes(grid, x, y, true)
        }
    }
}

fn solve(input: &str) -> (i32, i32) {
    let mut grid: Grid = [[Octopus::Flashing; GRID_DIMENSION]; GRID_DIMENSION];

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.char_indices() {
            grid[y][x] = Octopus::Normal(ch.to_digit(10).unwrap())
        }
    }

    let mut flashing = 0;

    for step in 1.. {
        for row in grid.iter_mut() {
            for octopus in row.iter_mut() {
                match octopus {
                    Octopus::Normal(value) => *value += 1,
                    _ => unreachable!(),
                }
            }
        }

        for y in 0..GRID_DIMENSION {
            for x in 0..GRID_DIMENSION {
                propagate_flashes(&mut grid, x, y, false);
            }
        }

        let mut all_flashing = true;
        for row in grid.iter_mut() {
            for octopus in row.iter_mut() {
                match octopus {
                    Octopus::Flashing => {
                        if step <= 100 {
                            flashing += 1;
                        }
                        *octopus = Octopus::Normal(0)
                    }
                    _ => all_flashing = false,
                }
            }
        }

        if all_flashing {
            return (flashing, step);
        }
    }

    unreachable!()
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

    time_it(|| println!("parts (1, 2): {:?}", solve(&input)));

    Ok(())
}
