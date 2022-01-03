use std::{
    collections::{BinaryHeap, HashSet},
    env, fs,
    path::Path,
    time::SystemTime,
};

type Coords = (i32, i32, i32);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    radius: i32,
}

impl Nanobot {
    fn from_str(line: &str) -> Self {
        let mut chars = line.chars().peekable();
        let mut values = [0; 4];

        for value in &mut values {
            while !matches!(chars.peek(), Some('-' | '0'..='9')) {
                chars.next();
            }

            let mut multiplier = 1;
            while matches!(chars.peek(), Some('-' | '0'..='9')) {
                match chars.next() {
                    Some('-') => multiplier = -1,
                    Some('0') => *value *= 10,
                    Some('1') => *value = (*value * 10) + 1,
                    Some('2') => *value = (*value * 10) + 2,
                    Some('3') => *value = (*value * 10) + 3,
                    Some('4') => *value = (*value * 10) + 4,
                    Some('5') => *value = (*value * 10) + 5,
                    Some('6') => *value = (*value * 10) + 6,
                    Some('7') => *value = (*value * 10) + 7,
                    Some('8') => *value = (*value * 10) + 8,
                    Some('9') => *value = (*value * 10) + 9,
                    _ => unreachable!(),
                }
            }

            *value *= multiplier;
        }

        Self {
            x: values[0],
            y: values[1],
            z: values[2],
            radius: values[3],
        }
    }

    fn distance_to(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn distance_to_coords(&self, other: &Coords) -> i32 {
        (self.x - other.0).abs() + (self.y - other.1).abs() + (self.z - other.2).abs()
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct State(Coords, i32);

impl State {
    fn distance_to_origin(&self) -> i32 {
        let (x, y, z) = self.0;
        x.abs() + y.abs() + z.abs()
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.1.cmp(&other.1) {
            std::cmp::Ordering::Equal => self
                .distance_to_origin()
                .cmp(&other.distance_to_origin())
                .reverse(),
            other => other,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part_one(input: &str) -> i32 {
    let nanobots: Vec<_> = input.lines().map(Nanobot::from_str).collect();

    let strongest_signal = nanobots.iter().max_by_key(|bot| bot.radius).unwrap();

    println!("{:?}", nanobots);
    println!("{:?}", strongest_signal);

    nanobots.iter().fold(0, |acc, bot| {
        acc + match strongest_signal
            .distance_to(bot)
            .cmp(&strongest_signal.radius)
        {
            std::cmp::Ordering::Greater => 0,
            _ => 1,
        }
    })
}

fn count_for_location(nanobots: &Vec<Nanobot>, location: Coords) -> i32 {
    let mut count = 0;

    for bot in nanobots {
        if bot.distance_to_coords(&location) <= bot.radius {
            count += 1;
        }
    }

    count
}

fn find_best(
    nanobots: &Vec<Nanobot>,
    start: Coords,
    to_move: i32,
    seen: &mut HashSet<(i32, i32, i32)>,
) -> Coords {
    let mut best_location = start;
    let mut highest_count = count_for_location(&nanobots, best_location);
    let mut heap = BinaryHeap::from([State(best_location, highest_count)]);

    while let Some(State(location, score)) = heap.pop() {
        if score < highest_count {
            continue;
        }

        if score > highest_count {
            best_location = location;
            highest_count = score;
        }

        for (dx, dy, dz) in [
            (to_move, 0, 0),
            (-to_move, 0, 0),
            (0, to_move, 0),
            (0, -to_move, 0),
            (to_move, to_move, 0),
            (to_move, -to_move, 0),
            (-to_move, to_move, 0),
            (-to_move, -to_move, 0),
            (0, 0, to_move),
            (to_move, 0, to_move),
            (-to_move, 0, to_move),
            (0, to_move, to_move),
            (0, -to_move, to_move),
            (to_move, to_move, to_move),
            (to_move, -to_move, to_move),
            (-to_move, to_move, to_move),
            (-to_move, -to_move, to_move),
            (0, 0, -to_move),
            (to_move, 0, -to_move),
            (-to_move, 0, -to_move),
            (0, to_move, -to_move),
            (0, -to_move, -to_move),
            (to_move, to_move, -to_move),
            (to_move, -to_move, -to_move),
            (-to_move, to_move, -to_move),
            (-to_move, -to_move, -to_move),
        ] {
            let new_location = (location.0 + dx, location.1 + dy, location.2 + dz);
            if !seen.insert(new_location) {
                continue;
            }
            let new_score = count_for_location(&nanobots, new_location);
            heap.push(State(new_location, new_score));
        }
    }

    best_location
}

// todo: use z3 to make it run faster than 20 minutes
fn part_two(input: &str) -> i32 {
    let nanobots: Vec<_> = input.lines().map(Nanobot::from_str).collect();

    let mut to_move = 100000;
    let mut best_location = (0, 0, 0);
    let mut seen = HashSet::from([best_location]);
    while to_move != 1 {
        best_location = find_best(&nanobots, best_location, to_move, &mut seen);
        to_move /= 10;
    }

    State(best_location, 0).distance_to_origin()
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
    // time_it(|| println!("part 2: {}", part_two(&input)));
    time_it(|| println!("part 2: {}", part_two_z3(&input)));

    Ok(())
}
