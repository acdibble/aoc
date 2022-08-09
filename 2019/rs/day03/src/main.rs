use std::{
    collections::{HashMap, HashSet},
    env, fs,
    path::Path,
    time::SystemTime,
};

struct WireIterator<'a> {
    steps: i32,
    parts: std::str::Split<'a, char>,
    x: i32,
    y: i32,
    direction: &'a str,
    remaining_steps: i32,
}

impl<'a> WireIterator<'a> {
    fn from(string: &'a str) -> Self {
        Self {
            parts: string.split(','),
            steps: 0,
            direction: "",
            x: 0,
            y: 0,
            remaining_steps: 0,
        }
    }
}

impl<'a> Iterator for WireIterator<'a> {
    type Item = ((i32, i32), i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_steps > 0 {
            // do nothing
        } else {
            let part = self.parts.next()?;
            self.direction = &part[0..1];
            self.remaining_steps = part[1..].parse().unwrap();
        }

        self.steps += 1;

        match self.direction {
            "R" => self.x += 1,
            "U" => self.y += 1,
            "L" => self.x -= 1,
            "D" => self.y -= 1,
            _ => unreachable!(),
        };

        self.remaining_steps -= 1;
        Some(((self.x, self.y), self.steps))
    }
}

fn part_one(input: &str) -> i32 {
    let mut lines = input.lines();
    let wire_a = WireIterator::from(lines.next().unwrap())
        .into_iter()
        .map(|(loc, _)| loc)
        .collect::<HashSet<(i32, i32)>>();
    let wire_b = WireIterator::from(lines.next().unwrap())
        .into_iter()
        .map(|(loc, _)| loc)
        .collect();

    wire_a
        .intersection(&wire_b)
        .into_iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

fn part_two(input: &str) -> i32 {
    let mut lines = input.lines();
    let wire_a = WireIterator::from(lines.next().unwrap());
    let wire_b = WireIterator::from(lines.next().unwrap());
    let mut fewest_steps = i32::MAX;

    let map_a: HashMap<(i32, i32), i32> = wire_a.into_iter().collect();

    for (loc, steps) in wire_b.into_iter() {
        if let Some(steps_a) = map_a.get(&loc) {
            fewest_steps = fewest_steps.min(steps + steps_a);
        }
    }

    fewest_steps
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

    time_it(|| println!("part 1: {}", part_one(input.trim_end())));
    time_it(|| println!("part 1: {}", part_two(input.trim_end())));

    Ok(())
}
