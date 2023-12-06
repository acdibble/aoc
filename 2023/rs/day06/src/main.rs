use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug)]
struct Boat {
    time: usize,
    distance: usize,
}

impl Boat {
    fn race(&self, time_held: usize) -> bool {
        let time_remaining = self.time - time_held;
        let velocity = time_held;
        let distance = velocity * time_remaining;
        distance > self.distance
    }

    fn race_all(&self) -> usize {
        (0..self.time)
            .into_iter()
            .fold(0, |acc, t| if self.race(t) { acc + 1 } else { acc })
    }
}

fn part_one() -> usize {
    let mut lines = DATA.trim().lines();

    let times = lines.next().unwrap().split_ascii_whitespace().skip(1);
    let dists = lines.next().unwrap().split_ascii_whitespace().skip(1);

    times
        .zip(dists)
        .map(|(time, dist)| {
            Boat {
                time: time.parse().unwrap(),
                distance: dist.parse().unwrap(),
            }
            .race_all()
        })
        .product()
}

fn part_two() -> usize {
    let mut lines = DATA.trim().lines();

    let mut times = lines.next().unwrap().split(": ").skip(1);
    let mut dists = lines.next().unwrap().split(": ").skip(1);

    Boat {
        time: times
            .next()
            .unwrap()
            .replacen(" ", "", usize::MAX)
            .parse()
            .unwrap(),
        distance: dists
            .next()
            .unwrap()
            .replacen(" ", "", usize::MAX)
            .parse()
            .unwrap(),
    }
    .race_all()
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
