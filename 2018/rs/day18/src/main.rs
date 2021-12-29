use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Open,
    Trees,
    Lumberyard,
}

impl Tile {
    fn from_char(ch: char) -> Self {
        match ch {
            '.' => Self::Open,
            '|' => Self::Trees,
            '#' => Self::Lumberyard,
            _ => unreachable!(),
        }
    }

    fn count_if(&self, tile: Tile) -> i32 {
        if *self == tile {
            1
        } else {
            0
        }
    }
}

fn inc(n: usize, max: usize) -> Option<usize> {
    match n + 1 {
        next if next == max => None,
        next => Some(next),
    }
}

fn dec(n: usize, _: usize) -> Option<usize> {
    match n {
        0 => None,
        _ => Some(n - 1),
    }
}

fn noop(n: usize, _: usize) -> Option<usize> {
    Some(n)
}

type NeighborFn = fn(usize, usize) -> Option<usize>;
const NEIGHBORS: [(NeighborFn, NeighborFn); 8] = [
    (inc, inc),
    (inc, noop),
    (inc, dec),
    (dec, inc),
    (dec, noop),
    (dec, dec),
    (noop, inc),
    (noop, dec),
];

fn find_counts(area: &Vec<Vec<Tile>>) -> (i32, i32) {
    area.iter().fold((0, 0), |running_total, row| {
        row.iter().fold(running_total, |(t, l), tile| {
            (
                t + tile.count_if(Tile::Trees),
                l + tile.count_if(Tile::Lumberyard),
            )
        })
    })
}

fn detect_cycle(past_values: &Vec<(usize, (i32, i32))>) -> (usize, usize) {
    for i in 0..past_values.len() {
        for j in (i + 1)..past_values.len() {
            let (step_one, value_one) = past_values[i];
            let (step_two, value_two) = past_values[j];

            if value_one != value_two {
                continue;
            }

            for k in 0..=10 {
                let (_, value_one) = past_values[i + k];
                let (_, value_two) = past_values[j + k];
                if value_one != value_two {
                    break;
                }

                if k == 10 {
                    return (step_one, step_two - step_one);
                }
            }
        }
    }

    unreachable!()
}

fn solve(input: &str, target: usize) -> i32 {
    let mut area: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect();
    let max_x = area[0].len();
    let max_y = area.len();

    let mut buffer = area.clone();

    let mut past_values = vec![];
    let mut seen = HashSet::new();

    for step in 0..target {
        for (y, row) in area.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let neighbors =
                    NEIGHBORS
                        .iter()
                        .filter_map(|(a, b)| match (a(x, max_x), b(y, max_y)) {
                            (Some(x), Some(y)) => Some(area[y][x]),
                            _ => None,
                        });

                match tile {
                    Tile::Open => {
                        let total = neighbors.fold(0, |acc, tile| acc + tile.count_if(Tile::Trees));
                        buffer[y][x] = if total >= 3 { Tile::Trees } else { *tile };
                    }
                    Tile::Trees => {
                        let total =
                            neighbors.fold(0, |acc, tile| acc + tile.count_if(Tile::Lumberyard));
                        buffer[y][x] = if total >= 3 { Tile::Lumberyard } else { *tile };
                    }
                    Tile::Lumberyard => {
                        buffer[y][x] = match neighbors.fold((0, 0), |(a, b), tile| {
                            (
                                a + tile.count_if(Tile::Lumberyard),
                                b + tile.count_if(Tile::Trees),
                            )
                        }) {
                            (0, _) | (_, 0) => Tile::Open,
                            _ => *tile,
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut area, &mut buffer);
        let value = find_counts(&area);
        past_values.push((step, value));
        seen.insert(value);

        if past_values.len() - seen.len() > 50 {
            break;
        }
    }

    if past_values.len() == target {
        let (_, (a, b)) = past_values.pop().unwrap();
        return a * b;
    }

    let (offset, cycle_length) = detect_cycle(&past_values);

    let index = (target - offset - 1) % cycle_length;

    let (_, (a, b)) = past_values[index + offset];

    a * b
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

    time_it(|| println!("part 1: {}", solve(&input, 10)));
    time_it(|| println!("part 2: {}", solve(&input, 1000000000)));

    Ok(())
}
