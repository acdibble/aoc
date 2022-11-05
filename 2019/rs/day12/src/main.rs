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

fn get_energy(Tuple { x, y, z, .. }: &Tuple) -> i32 {
    x.abs() + y.abs() + z.abs()
}

fn get_total_energy(vec: &Vec<Moon>) -> i32 {
    vec.iter()
        .map(|m| get_energy(&m.position) * get_energy(&m.velocity))
        .sum()
}

#[derive(Debug)]
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
}

fn parse_moons() -> Vec<Moon> {
    DATA.trim()
        .lines()
        .map(|line| {
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
            Moon {
                position: Tuple::point(x, y, z),
                velocity: Tuple::vector(0, 0, 0),
            }
        })
        .collect::<Vec<_>>()
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

    get_total_energy(&moons)
}

fn part_two() -> i32 {
    let mut moons = parse_moons();

    let mut previous_energy = 0;

    loop {
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

        let total = get_total_energy(&moons);
        println!("{}", total - previous_energy);
        previous_energy = total;
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
