use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

fn part_one() -> i32 {
    let mut current_total = 0;
    let mut max = 0;

    for line in DATA.lines() {
        match line.parse::<i32>() {
            Ok(value) => current_total += value,
            _ => {
                max = max.max(current_total);
                current_total = 0;
            }
        }
    }

    max = max.max(current_total);

    max
}

fn part_two() -> i32 {
    let mut current_total = 0;
    let mut totals = Vec::new();

    for line in DATA.lines() {
        match line.parse::<i32>() {
            Ok(value) => current_total += value,
            _ => {
                totals.push(current_total);
                current_total = 0;
            }
        }
    }

    totals.push(current_total);

    totals.sort();
    totals.reverse();

    totals[0..3].iter().fold(0, |acc, next| acc + next)
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
