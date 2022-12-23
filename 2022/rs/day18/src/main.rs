use std::{collections::HashSet, time::SystemTime};
use utils::Coord3D;

const DATA: &'static str = include_str!("../data.txt");

fn parse_line(line: &str) -> Coord3D {
    let mut parts = line.split(',');

    Coord3D(
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

fn part_one() -> i32 {
    let coords: Vec<_> = DATA.lines().map(parse_line).collect();

    let mut result = coords.len() as i32 * 6;

    for (i, coord) in coords.iter().enumerate() {
        for other in coords.iter().skip(i + 1) {
            if coord.adjacent(other) {
                result -= 2;
            }
        }
    }

    result
}

fn part_two() -> i32 {
    let lava_droplets: HashSet<_> = DATA.lines().map(parse_line).collect();

    let start = Coord3D(0, 0, 0);

    let mut stack = vec![start];
    let mut visited = HashSet::from([start]);
    let mut result = 0;

    let mut x_max = 0;
    let mut y_max = 0;
    let mut z_max = 0;
    let mut x_min = 0;
    let mut y_min = 0;
    let mut z_min = 0;

    for coord in &lava_droplets {
        x_max = x_max.max(coord.0 + 1);
        y_max = y_max.max(coord.1 + 1);
        z_max = z_max.max(coord.2 + 1);
        x_min = x_min.min(coord.0 - 1);
        y_min = y_min.min(coord.1 - 1);
        z_min = z_min.min(coord.2 - 1);
    }

    while let Some(location) = stack.pop() {
        for neighbor in [
            [1, 0, 0],
            [-1, 0, 0],
            [0, 1, 0],
            [0, -1, 0],
            [0, 0, 1],
            [0, 0, -1],
        ]
        .map(|t| location.translate(t))
        {
            if (-1..=x_max).contains(&neighbor.0)
                && (-1..=y_max).contains(&neighbor.1)
                && (-1..=z_max).contains(&neighbor.2)
            {
                if lava_droplets.contains(&neighbor) {
                    result += 1;
                } else if visited.insert(neighbor) {
                    stack.push(neighbor)
                }
            }
        }
    }

    result
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
