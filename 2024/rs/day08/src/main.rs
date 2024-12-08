use std::{
    collections::{HashMap, HashSet},
    time::SystemTime,
};
use utils::Coord;

const DATA: &'static str = include_str!("../data.txt");

fn parse_data() -> (HashMap<char, Vec<Coord>>, i32) {
    let mut max = 0;
    let mut antennae = HashMap::<char, Vec<Coord>>::new();

    for (y, line) in DATA.trim().lines().enumerate() {
        max = y as i32;

        for (x, ch) in line.char_indices() {
            if ch == '.' {
                continue;
            }

            antennae
                .entry(ch)
                .or_default()
                .push(Coord::new(x as i32, y as i32))
        }
    }

    (antennae, max + 1)
}

fn part_one() -> i32 {
    let (antennae, max) = parse_data();

    let mut spots = HashSet::new();

    // println!("{antennae:?}");

    for coords in antennae.values() {
        for (i, &a) in coords.iter().enumerate() {
            for &b in coords.iter().skip(i + 1) {
                let diff = a - b;

                for spot in [a + diff, b - diff] {
                    if spot.x >= 0 && spot.x < max && spot.y >= 0 && spot.y < max {
                        spots.insert(spot);
                    }
                }
            }
        }
    }

    // for y in 0..max {
    //     for x in 0..max {
    //         if spots.contains(&Coord::new(x, y)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!()
    // }

    spots.len() as i32
}

fn part_two() -> i32 {
    let (antennae, max) = parse_data();

    let mut spots = HashSet::new();

    // println!("{antennae:?}");

    for coords in antennae.values() {
        for (i, &a) in coords.iter().enumerate() {
            for &b in coords.iter().skip(i + 1) {
                let diff = a - b;
                spots.insert(a);
                spots.insert(b);
                for (mut spot, diff) in [(a, diff), (b, -diff)] {
                    loop {
                        spot += diff;
                        if spot.x >= 0 && spot.x < max && spot.y >= 0 && spot.y < max {
                            spots.insert(spot);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    for y in 0..max {
        for x in 0..max {
            if spots.contains(&Coord::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }

    spots.len() as i32
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
