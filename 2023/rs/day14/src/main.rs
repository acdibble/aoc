use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

const ROCK: char = 'O';
const EMPTY: char = '.';

struct CycleDetector<T: PartialEq + Eq> {
    values: Vec<T>,
    start: Option<usize>,
    len: Option<usize>,
}

impl<T: PartialEq + Eq> CycleDetector<T> {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            start: None,
            len: None,
        }
    }

    fn add(&mut self, value: T) {
        self.values.push(value);
    }

    fn at(&mut self, index: usize) -> Option<&T> {
        if self.values.get(index).is_some() {
            return self.values.get(index);
        }

        match (self.start, self.len) {
            (Some(start), Some(len)) => {
                let index_in_cycle = (index - start) % len;

                let cycle = &self.values[start..start + len];

                return cycle.get(index_in_cycle);
            }
            _ => {}
        }

        if self.detect_cycle() {
            self.at(index)
        } else {
            None
        }
    }

    fn detect_cycle(&mut self) -> bool {
        if self.start.is_some() {
            return true;
        }

        'outer: for (start, a) in self.values.iter().enumerate() {
            for (current, b) in self.values.iter().enumerate().skip(start + 1) {
                if a != b {
                    continue;
                }

                let mut offset = 1;

                while start + offset != current {
                    offset += 1;

                    match (
                        self.values.get(start + offset),
                        self.values.get(current + offset),
                    ) {
                        (Some(a), Some(b)) if a != b => continue 'outer,
                        (None, _) | (_, None) => continue 'outer,
                        _ => {}
                    }
                }

                if offset == 1 {
                    continue;
                }

                self.start = Some(start);
                self.len = Some(offset);
                return true;
            }
        }

        false
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn tilt_rocks(cycles: usize, directions: &[Direction]) -> usize {
    let mut map: Vec<Vec<_>> = DATA.trim().lines().map(|l| l.chars().collect()).collect();

    let y_len = map.len();
    let x_len = map[0].len();
    let mut detector = CycleDetector::new();

    for _ in 0..cycles.min(250) {
        for dir in directions {
            let mut run = true;
            while run {
                run = false;

                let it: Box<dyn Iterator<Item = (usize, usize)>> = match dir {
                    Direction::North => {
                        Box::from((1..y_len).flat_map(|y| (0..x_len).map(move |x| (x, y))))
                    }
                    Direction::South => Box::from(
                        (0..(y_len - 1))
                            .rev()
                            .flat_map(|y| (0..x_len).map(move |x| (x, y))),
                    ),
                    Direction::West => {
                        Box::from((1..x_len).flat_map(|x| (0..y_len).map(move |y| (x, y))))
                    }
                    Direction::East => Box::from(
                        (0..(x_len - 1))
                            .rev()
                            .flat_map(|x| (0..y_len).map(move |y| (x, y))),
                    ),
                };

                for (x, y) in it {
                    if map[y][x] != ROCK {
                        continue;
                    }

                    let next = match dir {
                        Direction::North => Some(&mut map[y - 1][x]),
                        Direction::East => Some(&mut map[y][x + 1]),
                        Direction::South => Some(&mut map[y + 1][x]),
                        Direction::West => Some(&mut map[y][x - 1]),
                    };

                    if let Some(next) = next {
                        if *next == EMPTY {
                            *next = ROCK;
                            map[y][x] = EMPTY;
                            run = true;
                        }
                    }
                }
            }
        }

        let mut line_value = y_len + 1;
        let mut total = 0;

        for line in &map {
            line_value -= 1;
            for ch in line {
                if *ch == ROCK {
                    total += line_value;
                }
            }
        }

        detector.add(total);
    }

    detector.at(cycles - 1).copied().unwrap()
}

fn part_one() -> usize {
    tilt_rocks(1, &[Direction::North])
}

fn part_two() -> usize {
    tilt_rocks(
        1000000000,
        &[
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ],
    )
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
