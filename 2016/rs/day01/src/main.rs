use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

enum Direction {
    Left,
    Right,
}
type Step = (Direction, i32);

fn part_one(input: &Vec<Step>) -> i32 {
    let mut bearing = 0;
    let mut northing = 0;
    let mut easting = 0;

    for step in input {
        match step.0 {
            Direction::Right => bearing = (bearing + 90) % 360,
            Direction::Left => bearing = (360 + bearing - 90) % 360,
        }

        match bearing {
            0 => northing += step.1,
            90 => easting += step.1,
            180 => northing -= step.1,
            270 => easting -= step.1,
            _ => unreachable!(),
        }
    }

    northing + easting
}

fn part_two(input: &Vec<Step>) -> i32 {
    let mut bearing = 0;
    let mut northing = 0;
    let mut easting = 0;
    let mut places_visited = HashSet::new();
    places_visited.insert((0, 0));

    for step in input {
        match step.0 {
            Direction::Right => bearing = (bearing + 90) % 360,
            Direction::Left => bearing = (360 + bearing - 90) % 360,
        }

        for _ in 0..step.1 {
            let (to_change, step_amount) = match bearing {
                0 => (&mut northing, 1),
                90 => (&mut easting, 1),
                180 => (&mut northing, -1),
                270 => (&mut easting, -1),
                _ => unreachable!(),
            };
            *to_change += step_amount;
            if !places_visited.insert((northing, easting)) {
                return northing + easting;
            }
        }
    }

    unreachable!()
}

fn time_it<F>(fun: F)
where
    F: Fn() -> (),
{
    let start = SystemTime::now();
    fun();
    println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros())
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    let directions = input
        .split(", ")
        .map(|step| {
            let mut it = step.chars();
            let left_or_right = it.next().unwrap();
            let amount = it.collect::<String>().parse().unwrap();
            match left_or_right {
                'L' => (Direction::Left, amount),
                'R' => (Direction::Right, amount),
                _ => unreachable!(),
            }
        })
        .collect();

    time_it(|| println!("part 1: {}", part_one(&directions)));
    time_it(|| println!("part 2: {}", part_two(&directions)));

    Ok(())
}
