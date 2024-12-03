use regex::Regex;
use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

fn part_one() -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(DATA)
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
        .sum()
}

fn part_two() -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(do)(\(\))|(don't)(\(\))").unwrap();

    let mut active = true;
    let mut sum = 0;

    for (_, [a, b]) in re.captures_iter(DATA).map(|c| c.extract()) {
        println!("{a} {b}");
        match a {
            "do" => active = true,
            "don't" => active = false,
            _ => {
                if active {
                    sum = sum + a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
                }
            }
        }
    }

    sum
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
