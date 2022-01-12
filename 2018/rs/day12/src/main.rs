use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum State {
    Dead,
    Live,
}

#[derive(Debug)]
struct Plant {
    location: i32,
    state: State,
}

impl Plant {
    fn new(location: i32, state: State) -> Self {
        Self { location, state }
    }
}

impl std::convert::TryFrom<char> for State {
    type Error = char;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '.' => Ok(Self::Dead),
            '#' => Ok(Self::Live),
            _ => Err(ch),
        }
    }
}

fn parse_combinations(lines: std::str::Lines) -> Vec<([State; 5], State)> {
    lines
        .map(|line| {
            let mut parts = line.split(" => ");
            let mut states = parts.next().unwrap().chars();

            (
                [
                    State::try_from(states.next().unwrap()).unwrap(),
                    State::try_from(states.next().unwrap()).unwrap(),
                    State::try_from(states.next().unwrap()).unwrap(),
                    State::try_from(states.next().unwrap()).unwrap(),
                    State::try_from(states.next().unwrap()).unwrap(),
                ],
                State::try_from(parts.next().unwrap().chars().next().unwrap()).unwrap(),
            )
        })
        .collect()
}

fn count_plants(plants: &VecDeque<Plant>) -> usize {
    plants.into_iter().fold(0, |acc, plant| {
        acc + match plant.state {
            State::Live => plant.location,
            _ => 0,
        }
    }) as usize
}

fn iterate(input: &str, times: usize) -> usize {
    let mut lines = input.lines();
    let mut plants: VecDeque<_> = lines
        .next()
        .unwrap()
        .split(' ')
        .nth(2)
        .unwrap()
        .chars()
        .flat_map(State::try_from)
        .enumerate()
        .map(|(loc, state)| Plant {
            state,
            location: loc as i32,
        })
        .collect();

    lines.next();

    let combinations = parse_combinations(lines);
    let mut buffer = VecDeque::with_capacity(combinations.len());
    let mut previous_amount = count_plants(&plants);
    let mut previous_diff = 0;

    for step in 0..times {
        let first = plants.front().unwrap().location;

        let mut current = [
            Plant::new(first - 4, State::Dead),
            Plant::new(first - 3, State::Dead),
            Plant::new(first - 2, State::Dead),
            Plant::new(first - 1, State::Dead),
            plants.pop_front().unwrap(),
        ];

        let last = plants.back().unwrap().location;

        plants.push_back(Plant::new(last + 1, State::Dead));
        plants.push_back(Plant::new(last + 2, State::Dead));
        plants.push_back(Plant::new(last + 3, State::Dead));

        while let Some(plant) = plants.pop_front() {
            current.rotate_left(1);
            current[4] = plant;
            let states = [
                current[0].state,
                current[1].state,
                current[2].state,
                current[3].state,
                current[4].state,
            ];

            for combo in &combinations {
                if combo.0 == states {
                    buffer.push_back(Plant::new(current[2].location, combo.1));
                    break;
                }
            }
        }

        std::mem::swap(&mut plants, &mut buffer);

        if times > 20 {
            let amount = count_plants(&plants);
            let diff = amount as i32 - previous_amount as i32;

            if diff == previous_diff {
                let remaining_generations = 50000000000usize - step - 1;
                return amount + remaining_generations * diff as usize;
            }

            previous_diff = diff;
            previous_amount = amount;
        }
    }

    count_plants(&plants)
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

    time_it(|| println!("part 1: {}", iterate(&input, 20)));
    time_it(|| println!("part 2: {}", iterate(&input, 500)));

    Ok(())
}
