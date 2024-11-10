use fraction::prelude::*;
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    ops::{Add, Mul, Sub},
    time::SystemTime,
};

type Fraction = GenericFraction<u128>;

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Stone {
    p: Tuple<i128>,
    v: Tuple<i128>,
}

impl From<&str> for Stone {
    fn from(value: &str) -> Self {
        let mut parts = value.split(" @ ");
        Self {
            p: Tuple::from(parts.next().unwrap()),
            v: Tuple::from(parts.next().unwrap()),
        }
    }
}

impl Sub for Stone {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            p: self.p - rhs.p,
            v: self.v - rhs.v,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Tuple<T>(T, T, T);

impl From<&str> for Tuple<i128> {
    fn from(value: &str) -> Self {
        let mut parts = value.split(",");
        let x = parts.next().unwrap().trim().parse().unwrap();
        let y = parts.next().unwrap().trim().parse().unwrap();
        let z = parts.next().unwrap().trim().parse().unwrap();
        Self(x, y, z)
    }
}

impl<T: Display> Debug for Tuple<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.0, self.1, self.2)
    }
}

impl<T> Tuple<T> {
    fn new((x, y, z): (T, T, T)) -> Self {
        Tuple(x, y, z)
    }
}

impl<T: Add<Output = T>> Add<Tuple<T>> for Tuple<T> {
    type Output = Self;

    fn add(self, rhs: Tuple<T>) -> Self::Output {
        Tuple(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: Sub<Output = T>> Sub<Tuple<T>> for Tuple<T> {
    type Output = Self;

    fn sub(self, rhs: Tuple<T>) -> Self::Output {
        Tuple(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Tuple<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Tuple<Fraction> {
    fn is_in_future(&self, other: &Tuple<i128>, velocity: &Tuple<i128>) -> bool {
        if velocity.0.is_positive() && self.0 < other.0.into() {
            return false;
        }

        if velocity.0.is_negative() && self.0 > other.0.into() {
            return false;
        }

        if velocity.1.is_positive() && self.1 < other.1.into() {
            return false;
        }

        if velocity.1.is_negative() && self.1 > other.1.into() {
            return false;
        }

        true
    }
}

impl Tuple<i128> {
    fn find_xy_line(&self, velocity: Self) -> (Fraction, Fraction) {
        let slope = Fraction::from(velocity.1) / Fraction::from(velocity.0);
        let intercept = Fraction::from(self.0) * slope - Fraction::from(self.1);

        (slope, -intercept)
    }
}

fn parse_lines() -> Vec<Stone> {
    DATA.trim().lines().map(Stone::from).collect()
}

fn part_one() -> i32 {
    let stones = parse_lines();

    let range = Fraction::from(200000000000000u64)..=Fraction::from(400000000000000u64);

    let mut count = 0;

    for (index, a) in stones.iter().enumerate() {
        let (slope_a, intercept_a) = a.p.find_xy_line(a.v);
        for b in stones.iter().skip(index + 1) {
            let (slope_b, intercept_b) = b.p.find_xy_line(b.v);
            if slope_a == slope_b {
                continue;
            }
            let x = (intercept_a + -intercept_b) / (slope_b + -slope_a);
            if !range.contains(&x) {
                continue;
            }

            let y = x * slope_a + intercept_a;

            if !range.contains(&y) {
                continue;
            }

            let intersection = Tuple::new((x, y, 0.into()));
            if intersection.is_in_future(&a.p, &a.v) && intersection.is_in_future(&b.p, &b.v) {
                count += 1;
            }
        }
    }

    count
}

fn part_two() -> i128 {
    let stones = parse_lines();

    // PotentialXSet = None
    let mut xs = None;
    // PotentialYSet = None
    let mut ys = None;
    // PotentialZSet = None
    let mut zs = None;
    // for A, B in it.combinations(InputList, 2):
    for (a, b) in stones
        .iter()
        .enumerate()
        .flat_map(|(i, a)| stones.iter().skip(i + 1).map(move |b| (a, b)))
    {
        if a.v.0 == b.v.0 && a.v.0.abs() > 100 {
            let mut set = HashSet::new();
            let diff = b.p.0 - a.p.0;
            for v in -1000..=1000 {
                if v == a.v.0 {
                    continue;
                }
                if diff % (v - a.v.0) == 0 {
                    set.insert(v);
                }
            }
            xs = match xs {
                None => Some(set),
                Some(existing) => Some(existing.intersection(&set).copied().collect()),
            }
        }
        if a.v.1 == b.v.1 && a.v.1.abs() > 100 {
            let mut set = HashSet::new();
            let diff = b.p.1 - a.p.1;
            for v in -1000..=1000 {
                if v == a.v.1 {
                    continue;
                }
                if diff % (v - a.v.1) == 0 {
                    set.insert(v);
                }
            }
            ys = match ys {
                None => Some(set),
                Some(existing) => Some(existing.intersection(&set).copied().collect()),
            }
        }
        if a.v.2 == b.v.2 && a.v.2.abs() > 100 {
            let mut set = HashSet::new();
            let diff = b.p.2 - a.p.2;
            for v in -1000..=1000 {
                if v == a.v.2 {
                    continue;
                }
                if diff % (v - a.v.2) == 0 {
                    set.insert(v);
                }
            }
            zs = match zs {
                None => Some(set),
                Some(existing) => Some(existing.intersection(&set).copied().collect()),
            }
        }
    }

    let x = xs.unwrap().into_iter().next().unwrap();
    let y = ys.unwrap().into_iter().next().unwrap();
    let z = zs.unwrap().into_iter().next().unwrap();

    let a = stones[0];
    let b = stones[1];

    let slope_a = Fraction::from(a.v.1 - y) / (a.v.0 - x);
    let slope_b = Fraction::from(b.v.1 - y) / (b.v.0 - x);
    let y_intercept_a = Fraction::from(a.p.1) - (slope_a * a.p.0);
    let y_intercept_b = Fraction::from(b.p.1) - (slope_b * b.p.0);
    let x_pos = (y_intercept_b - y_intercept_a) / (slope_a - slope_b);
    let y_pos = slope_a * x_pos + y_intercept_a;
    let t = (x_pos - a.p.0) / (a.v.0 - x);
    let z_pos = Fraction::from(a.v.2 - z) * t + a.p.2;

    (x_pos + y_pos + z_pos).try_into().unwrap()
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
