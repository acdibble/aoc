use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

fn concat(a: i64, b: i64) -> i64 {
    let mut shift = 1;
    let mut rem = b;
    while rem > 0 {
        shift *= 10;
        rem /= 10;
    }
    a * shift + b
}

enum Op {
    Add,
    Mul,
    Cat,
}

impl Op {
    fn perform(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Cat => concat(a, b),
        }
    }
}

fn solve(target: i64, values: &[i64], total: i64, ops: &[Op]) -> bool {
    if values.len() == 0 {
        return target == total;
    }

    ops.iter()
        .find(|op| solve(target, &values[1..], op.perform(total, values[0]), ops))
        .is_some()
}

fn check_line(line: &'static str, ops: &[Op]) -> Option<i64>
where
{
    let mut split = line.split(": ");
    let target = split.next().unwrap().parse().unwrap();

    let values: Vec<i64> = split
        .next()
        .unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    solve(target, &values, 0, ops).then_some(target)
}

fn part_one() -> i64 {
    let ops = [Op::Add, Op::Mul];
    DATA.trim().lines().flat_map(|l| check_line(l, &ops)).sum()
}

fn part_two() -> i64 {
    let ops = [Op::Add, Op::Mul, Op::Cat];
    DATA.trim().lines().flat_map(|l| check_line(l, &ops)).sum()
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
