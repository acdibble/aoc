use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn noop(value: usize) -> usize {
    value
}

fn inc(value: usize) -> usize {
    value + 1
}

fn dec(value: usize) -> usize {
    value - 1
}

const MOVEMENTS: [(fn(usize) -> usize, fn(usize) -> usize); 4] =
    [(noop, inc), (noop, dec), (inc, noop), (dec, noop)];

fn permute(size: usize) -> Vec<Vec<usize>> {
    let mut base_array: Vec<usize> = (0..size).into_iter().collect();
    let mut permutations = vec![base_array.clone()];
    let mut c = vec![0; size];
    let mut i = 0;

    while i < size {
        if c[i] < i {
            match i % 2 {
                0 => base_array.swap(0, i),
                _ => base_array.swap(c[i], i),
            }

            permutations.push(base_array.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    permutations
}

fn calculate_distance(input: &String, should_return: bool) -> usize {
    let mut number_locations = HashMap::new();
    let map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if matches!(c, '0'..='9') {
                        number_locations.insert(c, (x, y));
                    }

                    c
                })
                .collect()
        })
        .collect();

    let mut distances_map = HashMap::<usize, Vec<usize>>::new();
    let number_count = number_locations.len();

    for (&c, &(x, y)) in number_locations.iter() {
        let mut distances = vec![0; number_count];
        let mut seen = HashSet::from([(x, y)]);
        let mut states = VecDeque::from([(x, y, 0)]);

        while let Some((x, y, steps)) = states.pop_front() {
            match map.get(y) {
                Some(row) => match row.get(x) {
                    Some(c @ '0'..='9') => {
                        let number = c.to_digit(10).expect("failed to convert to digit") as usize;
                        distances[number] = steps;
                    }
                    _ => (),
                },
                _ => (),
            }

            for (x_action, y_action) in MOVEMENTS {
                let new_x = x_action(x);
                let new_y = y_action(y);
                if seen.contains(&(new_x, new_y)) {
                    continue;
                }
                match map.get(new_y) {
                    Some(row) => match row.get(new_x) {
                        Some('#') => (),
                        _ => {
                            seen.insert((new_x, new_y));
                            states.push_back((new_x, new_y, steps + 1))
                        }
                    },
                    _ => (),
                }
            }
        }

        distances_map.insert(c.to_digit(10).unwrap() as usize, distances);
    }

    let mut shortest = usize::MAX;

    for permutation in permute(number_count) {
        if permutation.first() != Some(&0) {
            continue;
        }
        let mut length = 0;
        for i in 0..number_count - 1 {
            match (permutation.get(i), permutation.get(i + 1)) {
                (Some(current), Some(next)) => match distances_map.get(current) {
                    Some(distances) => match distances.get(*next) {
                        Some(distance) => length += distance,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }

        if should_return {
            length += match permutation.last() {
                Some(number) => match distances_map.get(number) {
                    Some(distances) => match distances.first() {
                        Some(value) => value,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };
        }

        shortest = std::cmp::min(shortest, length);
    }

    shortest
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

    time_it(|| println!("part 1: {}", calculate_distance(&input, false)));
    time_it(|| println!("part 2: {}", calculate_distance(&input, true)));

    Ok(())
}
