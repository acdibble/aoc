use std::time::SystemTime;

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

fn run_program<F>(fun: F) -> Option<i32>
where
    F: Fn(usize, i32) -> Option<i32>,
{
    let mut x = 1;

    let mut ops = DATA.lines().map(Op::from);
    let mut in_progress_op = None;

    let mut result = None;

    for cycle in 0.. {
        match fun(cycle, x) {
            Some(value) => match &mut result {
                Some(total) => *total += value,
                opt => *opt = Some(value),
            },
            None => {}
        }

        match in_progress_op {
            None => match ops.next() {
                Some(Op::Noop) => {}
                op @ Some(Op::Addx(..)) => in_progress_op = op,
                None => break,
            },
            Some(Op::Addx(v)) => {
                in_progress_op = None;
                x += v
            }
            _ => unreachable!(),
        }
    }

    result
}

fn part_one() -> i32 {
    run_program(|cycle, x| {
        Some(if [20, 60, 100, 140, 180, 220].contains(&(cycle + 1)) {
            (cycle as i32 + 1) * x
        } else {
            0
        })
    })
    .unwrap()
}

fn part_two() {
    run_program(|cycle, x| {
        let x_position = cycle as i32 % 40;
        if x_position == 0 {
            println!()
        }

        if [x - 1, x, x + 1].contains(&x_position) {
            print!("#");
        } else {
            print!(".");
        }

        None
    });
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
