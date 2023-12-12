use std::{
    collections::{BTreeMap, BTreeSet},
    time::SystemTime,
};

const DATA: &'static str = include_str!("../data.txt");

type Point = (i64, i64);

fn manhattan_distance((a1, a2): &Point, (b1, b2): &Point) -> i64 {
    (a1 - b1).abs() + (a2 - b2).abs()
}

fn find_distances(times: i64) -> i64 {
    let mut empty_rows = BTreeSet::new();
    let mut empty_cols = BTreeSet::new();
    let mut occupied_cols = BTreeSet::new();
    let mut galaxies = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in DATA.trim().lines().enumerate() {
        let y = y as i64;
        max_y = y;
        empty_cols.insert(y);
        empty_rows.insert(y);
        for (x, ch) in line.char_indices() {
            let x = x as i64;
            max_x = x;
            if ch == '#' {
                occupied_cols.insert(x);
                empty_rows.remove(&(y));
                galaxies.push((x, y))
            }
        }
    }

    for n in occupied_cols {
        empty_cols.remove(&n);
    }

    let mut x_expansions = BTreeMap::new();

    for x in empty_cols {
        for n in x..=max_x {
            *x_expansions.entry(n).or_insert(0i64) += 1;
        }
    }

    let mut y_expansions = BTreeMap::new();

    for y in empty_rows {
        for n in y..=max_y {
            *y_expansions.entry(n).or_insert(0i64) += 1;
        }
    }

    for galaxy in &mut galaxies {
        let old = *galaxy;
        galaxy.0 += x_expansions.get(&old.0).unwrap_or(&0) * (times - 1).max(1);
        galaxy.1 += y_expansions.get(&old.1).unwrap_or(&0) * (times - 1).max(1);
    }

    let mut distances = 0;

    for (i, a) in galaxies.iter().enumerate() {
        for b in galaxies.iter().skip(i + 1) {
            distances += manhattan_distance(a, b);
        }
    }

    distances
}

fn part_one() -> i64 {
    find_distances(1)
}

fn part_two() -> i64 {
    find_distances(1_000_000)
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
