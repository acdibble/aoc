use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    time::SystemTime,
};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl From<&'static str> for Point {
    fn from(s: &'static str) -> Self {
        let mut parts = s.split(",");

        Self {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
        }
    }
}

impl Point {
    fn fall(&self) -> Option<Self> {
        if self.z == 1 {
            return None;
        }

        Some(Self {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        })
    }
}

#[derive(Debug)]
struct Brick {
    start: Point,
    end: Point,
}

impl From<&'static str> for Brick {
    fn from(s: &'static str) -> Self {
        let mut parts = s.split("~");

        Self {
            start: Point::from(parts.next().unwrap()),
            end: Point::from(parts.next().unwrap()),
        }
    }
}

impl Brick {
    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (self.start.x..=self.end.x).flat_map(move |x| {
            (self.start.y..=self.end.y)
                .flat_map(move |y| (self.start.z..=self.end.z).map(move |z| Point { x, y, z }))
        })
    }

    fn bottom_points(&self) -> impl Iterator<Item = Point> + '_ {
        self.points()
            .filter(|p| p.z == self.start.z.min(self.end.z))
    }

    fn fall(&self) -> Option<Self> {
        let start = self.start.fall()?;
        let end = self.end.fall()?;

        Some(Self { start, end })
    }
}

fn parse_bricks() -> Vec<Brick> {
    DATA.lines().map(Brick::from).collect()
}

fn build_support_system() -> (Vec<Brick>, HashMap<usize, HashSet<usize>>) {
    let mut bricks = parse_bricks();

    bricks.sort_by_cached_key(|brick| brick.start.z.min(brick.end.z));

    let mut points = HashMap::new();

    for (index, brick) in bricks.iter().enumerate() {
        for point in brick.points() {
            points.insert(point, index);
        }
    }

    'outer: for (index, brick) in bricks.iter_mut().enumerate() {
        while let Some(fallen) = brick.fall() {
            for point in fallen.bottom_points() {
                if points.contains_key(&point) {
                    continue 'outer;
                }
            }

            for point in brick.points() {
                points.remove(&point);
            }

            for point in fallen.points() {
                points.insert(point, index);
            }

            *brick = fallen;
        }
    }

    let mut supported_by = HashMap::new();

    for (index, brick) in bricks.iter().enumerate() {
        let supporting_bricks = supported_by.entry(index).or_insert_with(HashSet::new);

        for point in brick.bottom_points().flat_map(|p| p.fall()) {
            if let Some(other) = points.get(&point) {
                supporting_bricks.insert(*other);
            }
        }
    }

    (bricks, supported_by)
}

fn part_one() -> i32 {
    let (bricks, supported_by) = build_support_system();

    let mut count = 0;

    'outer: for i in 0..bricks.len() {
        for set in supported_by.values() {
            if set.contains(&i) && set.len() == 1 {
                continue 'outer;
            }
        }

        count += 1;
    }

    count
}

fn part_two() -> i32 {
    let (bricks, supported_by) = build_support_system();

    let mut count = 0;

    let mut cache = HashMap::new();

    for start in (0..bricks.len()).rev() {
        if let Some(amount) = cache.get(&start) {
            count += amount;
            continue;
        }

        let mut brick_count = 0;
        let mut queue = vec![start];
        let mut supported_by = supported_by.clone();

        while let Some(i) = queue.pop() {
            for (other, set) in supported_by.iter_mut() {
                if set.remove(&i) && set.is_empty() {
                    brick_count += 1;
                    queue.push(*other);
                }
            }
        }

        count += brick_count;
        cache.insert(start, brick_count);
    }

    count
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
