use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn get_char((x, y): (u32, u32), number: u32) -> char {
    let result = (x * x + 3 * x + 2 * x * y + y + y * y) + number;

    if result.count_ones() % 2 == 0 {
        '.'
    } else {
        '#'
    }
}

fn part_one(input: u32) -> i32 {
    let start = (1u32, 1u32);
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([(start, 0)]);

    let target_x = 31;
    let target_y = 39;

    let mut min_steps = i32::MAX;

    while let Some(((x, y), steps)) = queue.pop_front() {
        if steps > min_steps || seen.contains(&(x, y)) {
            continue;
        }

        seen.insert((x, y));

        if x == target_x && y == target_y {
            min_steps = std::cmp::min(min_steps, steps)
        }

        let left = (x.saturating_sub(1), y);
        let right = (x + 1, y);
        let up = (x, y.saturating_sub(1));
        let down = (x, y + 1);

        for coord in [left, right, up, down] {
            let next_char = get_char(coord, input);
            if next_char == '.' {
                queue.push_back((coord, steps + 1));
            }
        }
    }

    min_steps
}

fn part_two(input: u32) -> usize {
    let start = (1u32, 1u32);
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([(start, 0)]);

    while let Some(((x, y), steps)) = queue.pop_front() {
        if steps > 50 || seen.contains(&(x, y)) {
            continue;
        }

        seen.insert((x, y));

        let left = (x.saturating_sub(1), y);
        let right = (x + 1, y);
        let up = (x, y.saturating_sub(1));
        let down = (x, y + 1);

        for coord in [left, right, up, down] {
            let next_char = get_char(coord, input);
            if next_char == '.' {
                queue.push_back((coord, steps + 1));
            }
        }
    }

    seen.len()
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
    let input: u32 = fs::read_to_string(file_path)?.parse().unwrap();

    time_it(|| println!("part 1: {}", part_one(input)));
    time_it(|| println!("part 2: {}", part_two(input)));

    Ok(())
}
