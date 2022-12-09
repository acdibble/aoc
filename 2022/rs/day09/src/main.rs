use std::{collections::HashSet, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord(i32, i32);

impl Coord {
    fn is_touching(&self, other: &Self) -> bool {
        match ((self.0 - other.0).abs(), (self.1 - other.1).abs()) {
            (0, 0) | (1, 0) | (0, 1) | (1, 1) => true,
            _ => false,
        }
    }

    fn diff(&self, other: &Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

fn move_rope<const N: usize>() -> usize {
    let mut lines = DATA.lines();
    let mut rope = [Coord(0, 0); N];

    let mut positions = HashSet::<Coord>::new();

    while let Some(line) = lines.next() {
        let mut parts = line.split_ascii_whitespace();
        let dir = parts.next().unwrap();

        for _ in 0..parts.next().unwrap().parse().unwrap() {
            positions.insert(rope[N - 1]);
            let head = &mut rope[0];

            match dir {
                "U" => head.1 += 1,
                "R" => head.0 += 1,
                "D" => head.1 -= 1,
                "L" => head.0 -= 1,
                _ => unreachable!(),
            }

            for i in 0..(N - 1) {
                let j = i + 1;
                if !rope[i].is_touching(&rope[j]) {
                    let Coord(x, y) = rope[i].diff(&rope[j]);
                    let segment = &mut rope[j];
                    segment.0 += x.signum();
                    segment.1 += y.signum();
                }
            }
        }
    }

    positions.insert(rope[N - 1]);
    positions.len()
}

fn part_one() -> usize {
    move_rope::<2>()
}

fn part_two() -> usize {
    move_rope::<10>()
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
