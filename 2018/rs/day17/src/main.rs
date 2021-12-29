use std::{env, fs, path::Path, time::SystemTime};

#[derive(Debug, Copy, Clone)]
enum Tile {
    Sand,
    Clay,
    RestingWater,
    FlowingWater,
}

fn parse_numbers(line: &str) -> [usize; 3] {
    let mut result = [0; 3];

    let mut chars = line.chars().peekable();

    for num in 0..3 {
        while !matches!(chars.peek(), Some('0'..='9')) {
            chars.next();
        }

        while matches!(chars.peek(), Some('0'..='9')) {
            if let Some(ch) = chars.next() {
                result[num] *= 10;
                result[num] += ch.to_digit(10).unwrap() as usize
            }
        }
    }

    result
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    fn translate(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
        }
    }
}

#[inline(always)]
fn both(a: bool, b: bool) -> bool {
    a && b
}

fn flow(
    ground: &mut Vec<Vec<Tile>>,
    (x, y): (usize, usize),
    direction: Direction,
    overwrite: bool,
) -> bool {
    macro_rules! flow_to {
        ($direction:ident, $overwrite:ident) => {
            flow(
                ground,
                Direction::$direction.translate((x, y)),
                Direction::$direction,
                $overwrite,
            )
        };
    }

    if let Some(row) = ground.get_mut(y) {
        match row.get_mut(x) {
            Some(tile @ Tile::Sand) => *tile = Tile::FlowingWater,
            Some(tile @ Tile::FlowingWater) if overwrite => *tile = Tile::RestingWater,
            Some(Tile::FlowingWater) => return false,
            None => unreachable!(),
            _ => return true,
        }
    }

    match ground.get(y + 1) {
        Some(row) => match row.get(x) {
            Some(&Tile::Sand) => match direction {
                Direction::Down => {
                    flow(ground, direction.translate((x, y)), direction, overwrite)
                        && both(flow_to!(Left, overwrite), flow_to!(Right, overwrite))
                        && flow(ground, (x, y), direction, true)
                }
                _ => {
                    flow_to!(Down, overwrite)
                        && flow(ground, direction.translate((x, y)), direction, overwrite)
                }
            },
            Some(&Tile::Clay | Tile::RestingWater) => match direction {
                Direction::Down if !overwrite => {
                    both(flow_to!(Left, false), flow_to!(Right, false))
                        && flow(ground, (x, y), direction, true)
                }
                Direction::Down => flow_to!(Left, true) && flow_to!(Right, true),
                _ => flow(ground, direction.translate((x, y)), direction, overwrite),
            },
            _ => false,
        },
        _ => false,
    }
}

fn solve(input: &str) -> (i32, i32) {
    let mut ground: Vec<Vec<Tile>> = Vec::new();

    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_clay = usize::MAX;

    let ranges: Vec<_> = input
        .lines()
        .map(|line| {
            let [num1, num2, num3] = parse_numbers(line);

            let (x_range, y_range) = match &line[0..1] {
                "x" => ((num1..=num1), (num2..=num3)),
                "y" => ((num2..=num3), (num1..=num1)),
                _ => unreachable!(),
            };

            min_x = min_x.min(*x_range.start());
            max_x = max_x.max(*x_range.end());
            max_y = max_y.max(*y_range.end());

            (x_range, y_range)
        })
        .collect();

    min_x -= 5;
    max_x -= min_x - 5;

    for (x_range, y_range) in ranges {
        for y in y_range {
            min_clay = y.min(min_clay);
            for x in x_range.clone() {
                let x = x - min_x;
                while ground.len() <= y {
                    ground.push(Vec::new());
                }
                let row = ground.get_mut(y).unwrap();
                while row.len() <= x {
                    row.push(Tile::Sand);
                }
                row[x] = Tile::Clay;
            }
        }
    }

    for row in &mut ground {
        while row.len() <= max_x {
            row.push(Tile::Sand);
        }
    }

    flow(&mut ground, (500 - min_x, 1), Direction::Down, false);

    let mut flowing_water_count = 0;
    let mut resting_water_count = 0;

    for row in ground.into_iter().skip(min_clay) {
        for tile in row {
            match tile {
                Tile::FlowingWater => flowing_water_count += 1,
                Tile::RestingWater => resting_water_count += 1,
                _ => (),
            }
        }
    }

    (
        flowing_water_count + resting_water_count,
        resting_water_count,
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
