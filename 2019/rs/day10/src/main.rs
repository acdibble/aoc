use std::{
    cmp,
    collections::{HashMap, HashSet},
    time::SystemTime,
};
use utils::{fraction::Fraction, grid::Coordinate};

const DATA: &'static str = include_str!("../data.txt");

fn parse_asteroids() -> Vec<Coordinate> {
    let mut asteroids = vec![];

    for (y, line) in DATA.trim().lines().enumerate() {
        for (x, ch) in line.char_indices() {
            if ch == '#' {
                asteroids.push(Coordinate::new(x as i32, y as i32));
            }
        }
    }

    asteroids
}

fn find_station(asteroids: &Vec<Coordinate>) -> (Coordinate, usize) {
    let mut max = 0;
    let mut slopes = HashSet::new();
    let mut winner = Coordinate::new(0, 0);

    for coord in asteroids {
        slopes.clear();

        for other in asteroids {
            slopes.insert((other - coord).slope());
        }

        let count = slopes.len();

        if count > max {
            max = count;
            winner = *coord;
        }
    }

    (winner, max)
}

fn part_one() -> usize {
    let asteroids = parse_asteroids();

    find_station(&asteroids).1
}

fn part_two() -> i32 {
    let asteroids = parse_asteroids();

    let station = find_station(&asteroids).0;

    let mut quadrant_maps = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];
    let mut slopes = HashSet::new();

    for asteroid in asteroids {
        if asteroid == station {
            continue;
        }

        let mut coord = asteroid - station;
        let quadrant: usize;

        // (1,1)  |  (3,1)
        // -----(2,2)-----
        // (1,3)  |  (3,3)
        //
        // (-1,-1)  |  (1,-1)
        // -------(0,0)------
        // (-1,1)   |  (1,1)

        match (coord.x.cmp(&0), coord.y.cmp(&0)) {
            (cmp::Ordering::Equal | cmp::Ordering::Greater, cmp::Ordering::Less) => {
                quadrant = 0;
            }
            (cmp::Ordering::Greater, cmp::Ordering::Equal | cmp::Ordering::Greater) => {
                quadrant = 1;
                coord = coord.rotate_left();
            }
            (cmp::Ordering::Less | cmp::Ordering::Equal, cmp::Ordering::Greater) => {
                quadrant = 2;
                coord = coord.rotate_left().rotate_left();
            }
            (cmp::Ordering::Less, cmp::Ordering::Less | cmp::Ordering::Equal) => {
                quadrant = 3;
                coord = coord.rotate_right();
            }
            (cmp::Ordering::Equal, cmp::Ordering::Equal) => unreachable!(),
        }

        slopes.insert(coord.slope());

        quadrant_maps[quadrant]
            .entry(coord.slope())
            .or_insert_with(|| Vec::new())
            .push(coord)
    }

    let mut slopes = slopes.into_iter().collect::<Vec<_>>();
    slopes.sort_by(|a, b| {
        if matches!(a, Fraction::PositiveUndefined) {
            cmp::Ordering::Less
        } else if matches!(b, Fraction::PositiveUndefined) {
            cmp::Ordering::Greater
        } else {
            a.cmp(&b)
        }
    });

    for map in &mut quadrant_maps {
        for (_, vec) in map.iter_mut() {
            vec.sort_by(|a, b| {
                b.manhattan_distance_to_origin()
                    .cmp(&a.manhattan_distance_to_origin())
            })
        }
    }

    let mut destroyed = 0;

    loop {
        for (quadrant, map) in quadrant_maps.iter_mut().enumerate() {
            for slope in &slopes {
                if let Some(vec) = map.get_mut(slope) {
                    if let Some(el) = vec.pop() {
                        destroyed += 1;
                        if destroyed == 200 {
                            let mut result = el;
                            for _ in 0..quadrant {
                                result = result.rotate_right();
                            }
                            result += station;
                            return result.x * 100 + result.y;
                        }
                    }
                }
            }
        }
    }
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
