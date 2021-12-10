use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

type Grid = Vec<Vec<i32>>;

fn char_to_i32(ch: char) -> i32 {
    match ch {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        '0' => 0,
        _ => unreachable!(),
    }
}

fn inc(n: usize) -> Option<usize> {
    Some(n + 1)
}

fn dec(n: usize) -> Option<usize> {
    if n == 0 {
        None
    } else {
        Some(n - 1)
    }
}

fn noop(n: usize) -> Option<usize> {
    Some(n)
}

type NeighborFn = fn(usize) -> Option<usize>;

const NEIGHBORS: [(NeighborFn, NeighborFn); 4] =
    [(inc, noop), (dec, noop), (noop, inc), (noop, dec)];

fn solve(input: &str) -> (i32, i32) {
    let grid: Grid = input
        .lines()
        .map(|l| l.chars().map(char_to_i32).collect())
        .collect();

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut sizes = Vec::new();
    let mut risk_levels_sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            if value == 9 || seen.contains(&(x, y)) {
                continue;
            }

            queue.push_back((x, y));
            let mut size = 0;

            let mut lowest = value;

            while let Some(tuple) = queue.pop_front() {
                size += 1;

                for (x_fn, y_fn) in NEIGHBORS {
                    if let (Some(x), Some(y)) = (x_fn(tuple.0), y_fn(tuple.1)) {
                        if let Some(row) = grid.get(y) {
                            if let Some(&value) = row.get(x) {
                                if value != 9 && seen.insert((x, y)) {
                                    lowest = lowest.min(value);
                                    queue.push_back((x, y))
                                }
                            }
                        }
                    }
                }
            }

            sizes.push(size);
            risk_levels_sum += lowest + 1;
        }
    }

    sizes.sort();

    (
        risk_levels_sum,
        (0..3).fold(1, |acc, _| acc * sizes.pop().unwrap()),
    )
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
