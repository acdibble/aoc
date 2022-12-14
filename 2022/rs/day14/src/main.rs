use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

struct Cave {
    map: Vec<Vec<char>>,
}

impl Cave {
    fn new() -> Self {
        Self { map: Vec::new() }
    }

    fn from(data: &'static str) -> Self {
        let mut cave = Cave::new();
        let lines: Vec<Vec<_>> = data
            .lines()
            .map(|line| line.split(" -> ").map(Coord::from).collect())
            .collect();

        for mut line in lines {
            let mut current = line.pop().unwrap();
            while line.len() > 0 {
                *cave.get_mut(&current) = '#';
                let last = line.last().unwrap();
                current.move_towards(last);
                if current == *last {
                    line.pop();
                }
            }
            *cave.get_mut(&current) = '#';
        }

        cave
    }

    fn current_width(&self) -> usize {
        self.map.get(0).map(|row| row.len()).unwrap_or(0)
    }

    fn add_rows(&mut self, y: usize) {
        let width = self.current_width();

        while self.map.len() <= y {
            self.map.push(vec!['.'; width])
        }
    }

    fn add_cols(&mut self, x: usize) {
        let width = self.current_width();
        if width >= x + 1 {
            return;
        }

        let additional_width = x - self.current_width() + 1;

        if additional_width == 0 {
            return;
        }

        for row in &mut self.map {
            row.extend(['.'].iter().cycle().take(additional_width))
        }
    }

    fn get(&self, &Coord(x, y): &Coord) -> Option<&char> {
        self.map.get(y).and_then(|row| row.get(x))
    }

    fn get_mut(&mut self, &Coord(x, y): &Coord) -> &mut char {
        self.add_rows(y);
        self.add_cols(x);

        if let Some(row) = self.map.get_mut(y) {
            if let Some(ch) = row.get_mut(x) {
                return ch;
            }
        }

        unreachable!()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Coord(usize, usize);

impl From<&'static str> for Coord {
    fn from(pair: &'static str) -> Self {
        let mut parts = pair.split(',');

        Self(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        )
    }
}

impl Coord {
    fn move_towards(&mut self, other: &Self) {
        if self.0 == other.0 {
            if self.1 > other.1 {
                self.1 -= 1;
            } else {
                self.1 += 1;
            }
        } else {
            if self.0 > other.0 {
                self.0 -= 1;
            } else {
                self.0 += 1;
            }
        }
    }

    fn move_down(&mut self) {
        self.1 += 1;
    }

    fn move_left(&mut self) {
        self.0 -= 1;
    }

    fn move_right(&mut self) {
        self.0 += 1;
    }

    fn below_to_left(&self) -> Self {
        Self(self.0 - 1, self.1 + 1)
    }

    fn below_to_right(&self) -> Self {
        Self(self.0 + 1, self.1 + 1)
    }

    fn below(&self) -> Self {
        Self(self.0, self.1 + 1)
    }
}

fn flow_sand(mut cave: Cave) -> i32 {
    let mut result = 0;

    loop {
        let mut location = Coord(500, 0);

        loop {
            match cave.get(&location.below()) {
                Some('.') => {
                    location.move_down();
                    continue;
                }
                Some(_) => {}
                None => return result,
            }

            match cave.get(&location.below_to_left()) {
                Some('.') => {
                    location.move_down();
                    location.move_left();
                    continue;
                }
                Some(_) => {}
                None => return result,
            }

            match cave.get(&location.below_to_right()) {
                Some('.') => {
                    location.move_down();
                    location.move_right();
                    continue;
                }
                Some(_) => {}
                None => return result,
            }

            *cave.get_mut(&location) = 'o';
            result += 1;

            if location == Coord(500, 0) {
                return result;
            }

            break;
        }
    }
}

fn part_one() -> i32 {
    flow_sand(Cave::from(DATA))
}

fn part_two() -> i32 {
    let mut cave = Cave::from(DATA);
    cave.add_rows(cave.map.len() + 1);
    cave.add_cols(cave.current_width() * 2);

    for ch in cave.map.last_mut().unwrap().iter_mut() {
        *ch = '#'
    }

    flow_sand(cave)
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
