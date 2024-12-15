use std::{collections::HashMap, time::SystemTime};
use utils::{Coord, CopyRange};

use regex::Regex;

const DATA: &'static str = include_str!("../data.txt");

fn parse_data() -> impl Iterator<Item = (Coord, Coord)> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    DATA.lines().map(move |line| {
        let caps = re.captures(line).unwrap();
        (
            Coord::new(caps[1].parse().unwrap(), caps[2].parse().unwrap()),
            Coord::new(caps[3].parse().unwrap(), caps[4].parse().unwrap()),
        )
    })
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn part_one() -> i32 {
    let data = parse_data();

    let mut map = HashMap::new();

    for (p, v) in data {
        let mut x = (p.x + v.x * 100) % WIDTH;
        if x < 0 {
            x += WIDTH;
        }
        let mut y = (p.y + v.y * 100) % HEIGHT;
        if y < 0 {
            y += HEIGHT;
        }

        map.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
    }

    let left_x = CopyRange::new(0, WIDTH / 2 - 1);
    let right_x = CopyRange::new(WIDTH / 2 + 1, WIDTH);
    let bottom_y = CopyRange::new(0, HEIGHT / 2 - 1);
    let top_y = CopyRange::new(HEIGHT / 2 + 1, HEIGHT);

    let quad_1 = (right_x, bottom_y);
    let quad_2 = (right_x, top_y);
    let quad_3 = (left_x, top_y);
    let quad_4 = (left_x, bottom_y);

    let mut quad_map = HashMap::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if let Some(n) = map.get(&(x, y)) {
                if quad_1.0.contains(&x) && quad_1.1.contains(&y) {
                    *quad_map.entry(quad_1).or_insert(0) += n;
                } else if quad_2.0.contains(&x) && quad_2.1.contains(&y) {
                    *quad_map.entry(quad_2).or_insert(0) += n;
                } else if quad_3.0.contains(&x) && quad_3.1.contains(&y) {
                    *quad_map.entry(quad_3).or_insert(0) += n;
                } else if quad_4.0.contains(&x) && quad_4.1.contains(&y) {
                    *quad_map.entry(quad_4).or_insert(0) += n;
                }
            }
        }
    }

    quad_map.into_values().fold(1, |a, b| a * b)
}

fn part_two() -> i32 {
    let mut data: Vec<_> = parse_data().collect();

    let mut min = i32::MAX;
    let middle = Coord::new(WIDTH / 2, HEIGHT / 2);

    for i in 1.. {
        let mut sum = 0;

        for (p, v) in &mut data {
            p.x = (p.x + v.x) % WIDTH;
            if p.x < 0 {
                p.x += WIDTH;
            }
            p.y = (p.y + v.y) % HEIGHT;
            if p.y < 0 {
                p.y += HEIGHT;
            }

            sum += p.manhattan_distance(middle);
        }

        min = min.min(sum / data.len() as i32);

        if min == 30 {
            return i;
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

fn main() {
    time_it(|| println!("part 1: {}", part_one()));
    time_it(|| println!("part 2: {}", part_two()));
}
