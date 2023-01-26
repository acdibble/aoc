use std::{cmp, time::SystemTime};
use utils::{char_to_i32, tuples::Tuple};

const DATA: &'static str = include_str!("../data.txt");

fn get_change(a: &i32, b: &i32) -> i32 {
    match a.cmp(b) {
        cmp::Ordering::Greater => -1,
        cmp::Ordering::Equal => 0,
        cmp::Ordering::Less => 1,
    }
}

#[derive(Debug, Clone, Copy)]
struct Moon {
    position: Tuple,
    velocity: Tuple,
}

impl Moon {
    fn apply_gravity(&mut self, other: &Self) {
        self.velocity.x += get_change(&self.position.x, &other.position.x);
        self.velocity.y += get_change(&self.position.y, &other.position.y);
        self.velocity.z += get_change(&self.position.z, &other.position.z);
    }

    fn step(&mut self) {
        self.position += self.velocity;
    }

    fn energy(&self) -> i32 {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs())
            * (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs())
    }
}

fn parse_moons() -> [Moon; 4] {
    let mut moons = [Moon {
        position: Tuple::point(0, 0, 0),
        velocity: Tuple::vector(0, 0, 0),
    }; 4];

    for (index, line) in DATA.trim().lines().enumerate() {
        let mut chars = line.chars();
        chars.next();
        chars.next();
        chars.next();

        let mut nums = [0; 3];

        for num in &mut nums {
            let mut modifier = 1;
            match chars.next() {
                Some('-') => modifier = -1,
                Some(value) => *num += char_to_i32(value),
                _ => unreachable!(),
            }

            while let Some(ch) = chars.next() {
                if ch == ',' {
                    chars.next(); // ' '
                    chars.next(); // 'y/z'
                    chars.next(); // '='
                    break;
                } else if ch == '>' {
                    break;
                }

                *num *= 10;
                *num += char_to_i32(ch);
            }

            *num *= modifier;
        }
        let [x, y, z] = nums;

        moons[index].position = Tuple::point(x, y, z);
    }

    moons
}

fn part_one() -> i32 {
    let mut moons = parse_moons();

    for _ in 0..1000 {
        for i in 0..moons.len() {
            let (left, right) = moons.split_at_mut(i + 1);
            let moon = left.last_mut().unwrap();
            for other in right {
                moon.apply_gravity(other);
                other.apply_gravity(moon);
            }
        }

        for moon in &mut moons {
            moon.step();
        }
    }

    moons.into_iter().map(|m| m.energy()).sum()
}

struct CycleDetector(Vec<i32>);

impl CycleDetector {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn add(&mut self, value: i32) {
        self.0.push(value)
    }

    fn find_cycle(&mut self) -> Option<usize> {
        let needle = self.0.get(0)?;
        let values = &self.0;
        let mut search_index = 0;

        loop {
            search_index += 1;
            search_index += values.iter().skip(search_index).position(|x| x == needle)?;

            let end = search_index * 2;

            if end > values.len() {
                return None;
            }

            if &values[0..search_index] == &values[search_index..end] {
                return Some(search_index);
            }
        }
    }
}

struct MoonCycleDetector {
    x: CycleDetector,
    y: CycleDetector,
    z: CycleDetector,
}

impl MoonCycleDetector {
    fn new() -> Self {
        Self {
            x: CycleDetector::new(),
            y: CycleDetector::new(),
            z: CycleDetector::new(),
        }
    }

    fn add(&mut self, Tuple { x, y, z, .. }: &Tuple) {
        self.x.add(*x);
        self.y.add(*y);
        self.z.add(*z);
    }

    fn find_cycle(&mut self) -> Option<usize> {
        let x = self.x.find_cycle()?;
        let y = self.y.find_cycle()?;
        let z = self.z.find_cycle()?;

        Some(lcm(lcm(x, y), z))
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn find_cycle(detectors: &mut [MoonCycleDetector]) -> Option<usize> {
    let mut result = 1;

    for detector in detectors {
        let cycle = detector.find_cycle()?;
        result = lcm(result, cycle);
    }

    Some(result)
}

fn part_two() -> usize {
    let mut moons = parse_moons();

    let mut detectors = [
        MoonCycleDetector::new(),
        MoonCycleDetector::new(),
        MoonCycleDetector::new(),
        MoonCycleDetector::new(),
    ];

    loop {
        for _ in 0..10000 {
            for i in 0..moons.len() {
                detectors[i].add(&moons[i].position);

                let (left, right) = moons.split_at_mut(i + 1);
                let moon = left.last_mut().unwrap();
                for other in right {
                    moon.apply_gravity(other);
                    other.apply_gravity(moon);
                }
            }

            for moon in &mut moons {
                moon.step();
            }
        }

        if let Some(cycle) = find_cycle(&mut detectors) {
            return cycle;
        }
    }
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
