use std::{collections::HashMap, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

fn transform_value(value: &String) -> Box<dyn Iterator<Item = String>> {
    if value == "0" {
        Box::new(["1".to_owned()].into_iter())
    } else if value.len() % 2 == 0 {
        let half = value.len() / 2;
        let mut second: String = value[half..].chars().skip_while(|&ch| ch == '0').collect();
        if second.len() == 0 {
            second.push('0');
        }
        Box::new([value[0..half].to_owned(), second].into_iter())
    } else {
        let value = value.parse::<u64>().unwrap() * 2024;
        Box::new([value.to_string()].into_iter())
    }
}

fn blink(
    value: &str,
    counts: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<(String, usize), usize>,
    times: usize,
) -> usize {
    let values = counts.get(value).unwrap();

    if let Some(&result) = cache.get(&(value.to_owned(), times)) {
        return result;
    }

    let result = if times == 0 {
        values.len()
    } else {
        values
            .iter()
            .map(|v| blink(v, counts, cache, times - 1))
            .sum::<usize>()
    };

    cache.insert((value.to_owned(), times), result);

    result
}

fn solve(times: usize) -> usize {
    let input: Vec<_> = DATA
        .trim()
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect();
    let mut stack = input.clone();
    let mut steps = HashMap::new();

    while let Some(value) = stack.pop() {
        if steps.contains_key(&value) {
            continue;
        }

        let new_stones: Vec<_> = transform_value(&value).collect();
        steps.insert(value, new_stones.clone());
        stack.extend(new_stones);
    }

    let mut cache = HashMap::new();

    input
        .iter()
        .map(|v| blink(v, &steps, &mut cache, times))
        .sum()
}

fn part_one() -> usize {
    solve(24)
}

fn part_two() -> usize {
    solve(74)
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
