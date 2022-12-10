use std::{str::Lines, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

enum Op {
    Noop,
    Addx(i32),
}

impl From<&'static str> for Op {
    fn from(line: &'static str) -> Self {
        let mut parts = line.split_ascii_whitespace();
        match parts.next().unwrap() {
            "noop" => Op::Noop,
            "addx" => Op::Addx(parts.next().unwrap().parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

struct ProgramIter {
    lines: Lines<'static>,
    x: i32,
    cycle: i32,
    op: Option<Op>,
}

impl ProgramIter {
    fn new(data: &'static str) -> Self {
        Self {
            lines: data.lines(),
            x: 1,
            cycle: 0,
            op: None,
        }
    }
}

impl Iterator for ProgramIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let x = match self.op {
            None => match self.lines.next().map(Op::from) {
                Some(Op::Noop) => self.x,
                op @ Some(Op::Addx(..)) => {
                    self.op = op;
                    self.x
                }
                None => return None,
            },
            Some(Op::Addx(v)) => {
                self.op = None;
                let x = self.x;
                self.x += v;
                x
            }
            _ => unreachable!(),
        };

        let cycle = self.cycle;
        self.cycle += 1;
        Some((cycle, x))
    }
}

fn part_one() -> i32 {
    let mut result = 0;

    for (cycle, x) in ProgramIter::new(DATA) {
        let nth_cycle = cycle + 1;
        if [20, 60, 100, 140, 180, 220].contains(&nth_cycle) {
            result += nth_cycle * x
        }
    }

    result
}

fn part_two() {
    for (cycle, x) in ProgramIter::new(DATA) {
        let x_position = cycle as i32 % 40;
        if x_position == 0 {
            println!()
        }

        if [x - 1, x, x + 1].contains(&x_position) {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!()
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
    time_it(|| println!("part 2: {:?}", part_two()));
}
