use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

fn parse_range(range: &str) -> (i32, i32) {
    let mut parts = range.split('-');

    let first = parts.next().unwrap().parse().unwrap();
    let second = parts.next().unwrap().parse().unwrap();

    (first, second)
}

fn part_one() -> i32 {
    let mut result = 0;

    for line in DATA.lines() {
        let mut parts = line.split(',');

        let first = parse_range(parts.next().unwrap());
        let second = parse_range(parts.next().unwrap());

        if (first.0 <= second.0 && first.1 >= second.1)
            || (second.0 <= first.0 && second.1 >= first.1)
        {
            result += 1;
        }
    }

    result
}

fn part_two() -> i32 {
    let mut result = 0;

    for line in DATA.lines() {
        let mut parts = line.split(',');

        let first = parse_range(parts.next().unwrap());
        let second = parse_range(parts.next().unwrap());
        let first_range = first.0..=first.1;
        let second_range = second.0..=second.1;

        if first_range.contains(&second.0)
            || first_range.contains(&second.1)
            || second_range.contains(&first.0)
            || second_range.contains(&first.1)
        {
            result += 1;
        }
    }

    result
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
