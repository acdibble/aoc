use std::{collections::BTreeMap, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

const DAMAGED: char = '#';
const SPRING: char = '.';

fn place_spring(
    cache: &mut BTreeMap<(usize, usize), usize>,
    schema: &[char],
    groups: &[usize],
) -> usize {
    let cache_key = (schema.len(), groups.len());
    if let Some(count) = cache.get(&cache_key) {
        return *count;
    }

    if groups.is_empty() {
        let result = if schema.contains(&DAMAGED) { 0 } else { 1 };
        cache.insert(cache_key, result);
        return result;
    }

    let size = groups[0];
    let mut count = 0;

    for index in 0..schema.len() {
        if schema[0..index].contains(&DAMAGED) {
            break;
        }

        let schema = &schema[index..];

        if schema.len() < size {
            break;
        }

        let section = &schema[..size];
        if section.contains(&SPRING) || schema.get(size) == Some(&DAMAGED) {
            continue;
        }

        let skip = size + 1;

        if skip > schema.len() {
            if groups.len() == 1 {
                count += 1;
            }

            break;
        }

        let next_schema = &schema[skip..];
        count += place_spring(cache, next_schema, &groups[1..])
    }

    cache.insert(cache_key, count);

    count
}

fn solve_line(line: &'static str, unfold: bool) -> usize {
    let mut parts = line.split_ascii_whitespace();
    let schema_chars = parts.next().unwrap().chars();
    let group_nums = parts
        .next()
        .unwrap()
        .split(',')
        .flat_map(|x| x.parse::<usize>());

    let mut schema = Vec::new();
    let mut groups = Vec::new();

    let iterations = if unfold { 5 } else { 1 };

    for it in 0..iterations {
        if it != 0 {
            schema.push('?');
        }

        for ch in schema_chars.clone() {
            schema.push(ch);
        }

        for n in group_nums.clone() {
            groups.push(n);
        }
    }

    let mut cache = BTreeMap::new();
    place_spring(&mut cache, &mut schema, &groups)
}

fn part_one() -> usize {
    DATA.lines().map(|line| solve_line(line, false)).sum()
}

fn part_two() -> usize {
    DATA.lines().map(|line| solve_line(line, true)).sum()
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
