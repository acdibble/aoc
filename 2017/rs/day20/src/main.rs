use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn magnitude(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl std::str::FromStr for Coord {
    type Err = String;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut triples = [(0, 0); 3];
        let mut current = 0;

        for (index, c) in string.char_indices() {
            match c {
                ',' => {
                    triples[current].1 = index;
                    current += 1;
                    triples[current].0 = index + 1
                }
                'p' | 'v' | 'a' | '=' => (),
                '<' => triples[0].0 = index + 1,
                '>' => triples[2].1 = index,
                _ => (),
            }
        }

        let [(x1, x2), (y1, y2), (z1, z2)] = triples;

        Ok(Coord {
            x: string[x1..x2].parse().or_else(|_| Err(string.to_owned()))?,
            y: string[y1..y2].parse().or_else(|_| Err(string.to_owned()))?,
            z: string[z1..z2].parse().or_else(|_| Err(string.to_owned()))?,
        })
    }
}

#[derive(Debug)]
struct Particle {
    id: usize,
    position: Coord,
    velocity: Coord,
    acceleration: Coord,
}

impl Particle {
    fn tick(&mut self) {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
}

impl std::str::FromStr for Particle {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut parts = string.split(", ");

        Ok(Self {
            id: 0,
            position: parts.next().ok_or_else(|| string.to_owned())?.parse()?,
            velocity: parts.next().ok_or_else(|| string.to_owned())?.parse()?,
            acceleration: parts.next().ok_or_else(|| string.to_owned())?.parse()?,
        })
    }
}

fn part_one(input: &str) -> usize {
    let mut particles: Vec<Particle> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let mut particle = l.parse::<Particle>().unwrap();
            particle.id = i;
            particle
        })
        .collect();

    particles.sort_by(
        |a, b| match a.acceleration.magnitude().cmp(&b.acceleration.magnitude()) {
            Ordering::Equal => match a.velocity.magnitude().cmp(&b.velocity.magnitude()) {
                Ordering::Equal => a.position.magnitude().cmp(&b.position.magnitude()),
                ordering => ordering,
            },
            ordering => ordering,
        },
    );

    particles.first().unwrap().id
}

fn part_two(input: &str) -> usize {
    let mut particles: Vec<Particle> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let mut particle = l.parse::<Particle>().unwrap();
            particle.id = i;
            particle
        })
        .collect();

    let mut position_map = HashMap::<Coord, i32>::new();

    let mut len = particles.len();
    let mut unchanged_count = 0;

    while unchanged_count < 100 {
        for p in &particles {
            *position_map.entry(p.position).or_default() += 1;
        }

        particles.retain(|p| position_map.get(&p.position) == Some(&1));

        if particles.len() == len {
            unchanged_count += 1;
        } else {
            unchanged_count = 0;
        }

        len = particles.len();

        for p in &mut particles {
            p.tick()
        }
    }

    particles.len()
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
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}
