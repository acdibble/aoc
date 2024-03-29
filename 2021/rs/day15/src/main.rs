use std::collections::BinaryHeap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn inc(n: usize, max: usize) -> Option<usize> {
    match n + 1 {
        v if v == max => None,
        v => Some(v),
    }
}

fn dec(n: usize) -> Option<usize> {
    match n {
        0 => None,
        _ => Some(n - 1),
    }
}

struct State(u32, (usize, usize));

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

fn find_lowest_risk(grid: &Vec<Vec<u32>>) -> u32 {
    let grid_size = grid.len();

    let mut costs_matrix = vec![vec![0; grid_size]; grid_size];

    let mut heap = BinaryHeap::from([State(0, (0, 0))]);

    let target = (grid_size - 1, grid_size - 1);

    while let Some(State(amount, coords)) = heap.pop() {
        let (x, y) = coords;

        let cost = &mut costs_matrix[y][x];

        if *cost != 0 && *cost <= amount {
            continue;
        }

        *cost = amount;

        if coords == target {
            break;
        }

        if let Some(x) = inc(x, grid_size) {
            heap.push(State(amount + grid[y][x], (x, y)))
        }

        if let Some(y) = inc(y, grid_size) {
            heap.push(State(amount + grid[y][x], (x, y)))
        }

        if let Some(x) = dec(x) {
            heap.push(State(amount + grid[y][x], (x, y)))
        }

        if let Some(y) = dec(y) {
            heap.push(State(amount + grid[y][x], (x, y)))
        }
    }

    costs_matrix[target.1][target.0]
}

fn part_one(input: &str) -> u32 {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().flat_map(|ch| ch.to_digit(10)).collect())
        .collect();

    find_lowest_risk(&grid)
}

fn part_two(input: &str) -> u32 {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().flat_map(|ch| ch.to_digit(10)).collect())
        .collect();

    let original_len = grid.len();

    let mut enlarged: Vec<Vec<_>> = grid
        .iter()
        .map(|row| {
            let mut new_row = Vec::with_capacity(row.len() * 5);

            for growth in 0..5 {
                for num in row {
                    new_row.push(match num + growth {
                        n @ 1..=9 => n,
                        n => n - 9,
                    });
                }
            }

            new_row
        })
        .collect();

    for _ in 0..4 {
        let start_len = enlarged.len();
        for i in (1..=original_len).rev() {
            let mut new_row = enlarged[start_len - i].clone();
            for num in &mut new_row {
                *num = match *num + 1 {
                    n @ 1..=9 => n,
                    _ => 1,
                }
            }
            enlarged.push(new_row);
        }
    }

    find_lowest_risk(&enlarged)
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
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}
