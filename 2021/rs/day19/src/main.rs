use std::{
    collections::{HashMap, VecDeque},
    env, fs,
    path::Path,
    time::SystemTime,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
    original: [i32; 3],
    state: u8,
}

impl Beacon {
    fn from_str(line: &str) -> Self {
        let mut coord_part = line.split(',');
        let original = [
            coord_part.next().unwrap().parse().unwrap(),
            coord_part.next().unwrap().parse().unwrap(),
            coord_part.next().unwrap().parse().unwrap(),
        ];

        Self {
            x: original[0],
            y: original[1],
            z: original[2],
            original,
            state: 0,
        }
    }

    fn translate(&mut self, [dx, dy, dz]: &[i32; 3]) {
        self.state = 0;
        self.x += dx;
        self.y += dy;
        self.z += dz;
        self.original = [self.x, self.y, self.z];
    }

    fn enter_state(&mut self, state: u8) {
        self.state = match state {
            0 => 23,
            _ => state - 1,
        };

        self.transform();
    }

    fn transform(&mut self) {
        let [a, b, c] = self.original;

        let new_values = match self.state {
            0 => [a, b, c],
            1 => [a, c, -b],
            2 => [a, -b, -c],
            3 => [a, -c, b],
            4 => [b, a, -c],
            5 => [b, c, a],
            6 => [b, -a, c],
            7 => [b, -c, -a],
            8 => [c, a, b],
            9 => [c, b, -a],
            10 => [c, -a, -b],
            11 => [c, -b, a],
            12 => [-a, b, -c],
            13 => [-a, c, b],
            14 => [-a, -b, c],
            15 => [-a, -c, -b],
            16 => [-b, a, c],
            17 => [-b, c, -a],
            18 => [-b, -a, -c],
            19 => [-b, -c, a],
            20 => [-c, a, -b],
            21 => [-c, b, a],
            22 => [-c, -a, b],
            23 => [-c, -b, -a],
            _ => unreachable!(),
        };

        self.x = new_values[0];
        self.y = new_values[1];
        self.z = new_values[2];
        self.state = (self.state + 1) % 24;
    }
}

impl std::ops::Sub<Beacon> for Beacon {
    type Output = [i32; 3];
    fn sub(self, other: Self) -> Self>::Output {
        let [x1, y1, z1] = self.original;
        [x1 - other.x, y1 - other.y, z1 - other.z]
    }
}

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn from_lines(lines: &mut std::str::Lines) -> Self {
        let mut beacons = vec![];

        while let Some(line) = lines.next() {
            if line == "" {
                break;
            }

            beacons.push(Beacon::from_str(line));
        }

        Self { beacons }
    }

    fn enter_state(&mut self, state: u8) {
        for beacon in &mut self.beacons {
            beacon.enter_state(state);
        }
    }

    fn transform(&mut self) {
        for beacon in &mut self.beacons {
            beacon.transform();
        }
    }

    fn translate(&mut self, coords: &[i32; 3]) {
        for beacon in &mut self.beacons {
            beacon.translate(coords);
        }
    }

    fn count_vectors(&self, other: &Self) -> HashMap<[i32; 3], i32> {
        let mut counts = HashMap::new();

        for beacon in &self.beacons {
            for other in &other.beacons {
                *counts.entry(*beacon - *other).or_default() += 1;
            }
        }

        counts
    }

    fn combine(&mut self, other: Self) {
        for other in other.beacons {
            if !self.beacons.contains(&other) {
                self.beacons.push(other);
            }
        }
    }
}

fn parse_scanners(input: &str) -> VecDeque<Scanner> {
    let mut output = VecDeque::new();
    let mut lines = input.lines();

    while let Some(_) = lines.next() {
        output.push_back(Scanner::from_lines(&mut lines));
    }

    output
}

fn solve(input: &str) -> (usize, i32) {
    let mut scanners = parse_scanners(input);

    let mut zero = scanners.pop_front().unwrap();

    let mut scanner_locations = Vec::new();

    while let Some(mut scanner) = scanners.pop_front() {
        let mut max = 0;
        let mut max_state = 0;
        let mut vector = [0; 3];

        for state in 0..24 {
            let vectors = zero.count_vectors(&scanner);
            let (current_vector, current_max) =
                vectors.into_iter().max_by_key(|(_, count)| *count).unwrap();

            if current_max > max {
                max = current_max;
                max_state = state;
                vector = current_vector;
            }

            scanner.transform();
        }

        if max < 12 {
            scanners.push_back(scanner);
            continue;
        }

        scanner.enter_state(max_state);

        scanner.translate(&vector);
        zero.combine(scanner);
        scanner_locations.push(vector);
    }

    let mut furthest = 0;

    for [x1, y1, z1] in &scanner_locations {
        for [x2, y2, z2] in &scanner_locations {
            furthest = furthest.max((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs());
        }
    }

    (zero.beacons.len(), furthest)
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

    time_it(|| println!("parts 1, 2: {:?}", solve(&input)));

    Ok(())
}
