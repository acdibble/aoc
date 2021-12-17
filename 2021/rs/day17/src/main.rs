use std::cmp::Ordering;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

struct Probe {
    x_velocity: i32,
    x_position: i32,
    y_velocity: i32,
    y_position: i32,
    x_target: (i32, i32),
    y_target: (i32, i32),
    max_height: i32,
}

impl Probe {
    fn new(x_velocity: i32, y_velocity: i32, x_target: (i32, i32), y_target: (i32, i32)) -> Self {
        Probe {
            x_velocity,
            y_velocity,
            x_target,
            y_target,
            x_position: 0,
            y_position: 0,
            max_height: 0,
        }
    }

    fn step(&mut self) {
        self.x_position += self.x_velocity;
        self.y_position += self.y_velocity;
        self.max_height = self.max_height.max(self.y_position);

        match self.x_velocity.cmp(&0) {
            Ordering::Equal => (),
            Ordering::Greater => self.x_velocity -= 1,
            Ordering::Less => self.x_velocity += 1,
        }

        self.y_velocity -= 1;
    }

    fn hits_target(&mut self) -> bool {
        while self.x_position <= self.x_target.1 && self.y_position >= self.y_target.0 {
            if self.x_position >= self.x_target.0 && self.y_position <= self.y_target.1 {
                return true;
            }

            self.step();
        }

        false
    }
}

fn parse_target(input: &str) -> ((i32, i32), (i32, i32)) {
    let mut parts = input.split_ascii_whitespace();

    let x = parts.nth(2).unwrap();
    let mut x = x[2..x.len() - 1].split("..");
    let mut y = parts.next().unwrap()[2..].split("..");

    (
        (
            x.next().unwrap().parse().unwrap(),
            x.next().unwrap().parse().unwrap(),
        ),
        (
            y.next().unwrap().parse().unwrap(),
            y.next().unwrap().parse().unwrap(),
        ),
    )
}

fn solve(input: &str) -> (i32, i32) {
    let ((x1, x2), (y1, y2)) = parse_target(input);

    let mut max_height = i32::MIN;
    let mut hits = 0;

    for y_velocity in y1..(y1.abs() + y1.abs() / 2) {
        for x_velocity in 0..(x2 + x2 / 2) {
            let mut probe = Probe::new(x_velocity, y_velocity, (x1, x2), (y1, y2));
            if probe.hits_target() {
                max_height = max_height.max(probe.max_height);
                hits += 1;
            }
        }
    }

    (max_height, hits)
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
