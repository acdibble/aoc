use std::{cmp::Ordering, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

fn is_safe(report: &Vec<i32>) -> bool {
    let mut prev = None::<i32>;
    let mut dir = None::<Ordering>;

    for &int in report {
        if let Some(prev) = prev {
            if let Some(dir) = dir {
                if dir != prev.cmp(&int) {
                    return false;
                }
            } else {
                dir = Some(prev.cmp(&int));
            }

            if !(1..=3).contains(&(prev - int).abs()) {
                return false;
            }
        }

        prev = Some(int);
    }

    true
}

fn parse_reports() -> impl Iterator<Item = Vec<i32>> {
    DATA.lines().map(|l| {
        l.split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    })
}

fn part_one() -> i32 {
    parse_reports().fold(0, |acc, l| acc + if is_safe(&l) { 1 } else { 0 })
}

fn can_be_made_safe(report: Vec<i32>) -> bool {
    if is_safe(&report) {
        return true;
    }

    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);

        if is_safe(&new_report) {
            return true;
        }
    }

    false
}

fn part_two() -> i32 {
    parse_reports().fold(0, |acc, l| acc + if can_be_made_safe(l) { 1 } else { 0 })
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
