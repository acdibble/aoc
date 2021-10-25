use std::env;
use std::fs;
use std::path::Path;

struct Reindeer {
    speed: i32,
    flight_seconds: i32,
    rest_seconds: i32,
    distance_traveled: i32,
    time_traveled: i32,
}

impl Reindeer {
    fn from(string: &str) -> Reindeer {
        let mut it = string.split_ascii_whitespace();

        it.next().unwrap(); // name
        it.next(); // can
        it.next(); // fly

        let speed: i32 = it.next().unwrap().parse().unwrap();

        it.next(); // km/s
        it.next(); // for

        let flight_seconds: i32 = it.next().unwrap().parse().unwrap();

        it.next(); // seconds
        it.next(); // but
        it.next(); // then
        it.next(); // must
        it.next(); // rest
        it.next(); // for

        let rest_seconds: i32 = it.next().unwrap().parse().unwrap();

        Reindeer {
            speed,
            flight_seconds,
            rest_seconds,
            distance_traveled: 0,
            time_traveled: 0,
        }
    }

    fn travel(&mut self, seconds: i32) -> i32 {
        let cycle_time = self.rest_seconds + self.flight_seconds;

        for _ in 0..seconds {
            let remainder = self.time_traveled % cycle_time;
            let in_travel_mode = remainder < self.flight_seconds;
            if in_travel_mode {
                self.distance_traveled += self.speed;
            }
            self.time_traveled += 1;
        }

        self.distance_traveled
    }

    fn reset(&mut self) {
        self.distance_traveled = 0;
        self.time_traveled = 0;
    }
}

fn part_one(reindeers: &mut Vec<Reindeer>) -> i32 {
    let mut farthest_distance = 0;

    for reindeer in reindeers {
        let distance_traveled = reindeer.travel(2503);
        farthest_distance = std::cmp::max(farthest_distance, distance_traveled);
    }

    farthest_distance
}

fn part_two(reindeers: &mut Vec<Reindeer>) -> i32 {
    let mut distances = vec![0; reindeers.len()];
    let mut scoreboard = vec![0i32; reindeers.len()];
    let mut current_leader = 0;

    for _ in 0..2503 {
        for (index, reindeer) in reindeers.iter_mut().enumerate() {
            let distance_traveled = reindeer.travel(1);
            distances[index] = distance_traveled;
            current_leader = std::cmp::max(current_leader, distance_traveled);
        }

        for (index, distance) in distances.iter().enumerate() {
            if *distance == current_leader {
                scoreboard[index] += 1;
            }
        }
    }

    scoreboard.into_iter().max().unwrap()
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    let mut reindeers = input.lines().map(Reindeer::from).collect();

    println!("part 1: {}", part_one(&mut reindeers));

    for reindeer in &mut reindeers {
        reindeer.reset();
    }

    println!("part 2: {}", part_two(&mut reindeers));

    Ok(())
}
