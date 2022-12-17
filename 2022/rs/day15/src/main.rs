use std::{time::SystemTime, vec};
use utils::Coord;

const DATA: &'static str = include_str!("../data.txt");

struct Sensor {
    location: Coord,
    distance: i64,
}

impl Sensor {
    fn in_range(&self, coord: &Coord) -> bool {
        self.location.distance_to(&coord) <= self.distance
    }
}

fn prepare_data() -> (Vec<Sensor>, Vec<Coord>) {
    let mut sensors = vec![];
    let mut beacons = vec![];

    for mut parts in DATA.lines().map(|l| {
        l.split_ascii_whitespace()
            .filter(|part| part.starts_with("x") || part.starts_with("y"))
    }) {
        let sensor_x = parts.next().unwrap();
        let sensor_y = parts.next().unwrap();
        let beacon_x = parts.next().unwrap();
        let beacon_y = parts.next().unwrap();

        let location = Coord(
            sensor_x[2..sensor_x.len() - 1].parse().unwrap(),
            sensor_y[2..sensor_y.len() - 1].parse().unwrap(),
        );
        let beacon = Coord(
            beacon_x[2..beacon_x.len() - 1].parse().unwrap(),
            beacon_y[2..].parse().unwrap(),
        );
        sensors.push(Sensor {
            distance: location.distance_to(&beacon),
            location,
        });
        beacons.push(beacon);
    }

    (sensors, beacons)
}

fn part_one() -> i64 {
    let (sensors, beacons) = prepare_data();

    let mut min_x = 0;
    let mut max_x = 0;

    for sensor in &sensors {
        min_x = min_x.min(sensor.location.0 - sensor.distance);
        max_x = max_x.max(sensor.location.0 + sensor.distance);
    }

    let mut result = max_x - min_x;

    'outer: for x in min_x..=max_x {
        let loc = Coord(x, 2000000);

        for sensor in &sensors {
            if beacons.contains(&loc) || sensor.location == loc || sensor.in_range(&loc) {
                continue 'outer;
            }
        }

        result -= 1;
    }

    result
}

fn part_two() -> i64 {
    let (sensors, _) = prepare_data();

    let max_range = 4000000;

    for sensor in &sensors {
        let y_range = sensor.distance + 1;

        for dy in -y_range..=y_range {
            let x_location = y_range - dy.abs();

            'x_loop: for dx in [-x_location, x_location] {
                let location = sensor.location + Coord(dx, dy);
                if location.0 < 0
                    || location.1 < 0
                    || location.0 > max_range
                    || location.1 > max_range
                {
                    continue;
                }

                for other in &sensors {
                    if other.in_range(&location) {
                        continue 'x_loop;
                    }
                }

                return location.0 * 4000000 + location.1;
            }
        }
    }

    unreachable!()
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
