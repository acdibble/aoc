use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
struct Disc {
    position_count: usize,
    starting_position: usize,
}

impl Disc {
    fn get_position(&self, time: usize) -> usize {
        (self.starting_position + time) % self.position_count
    }
}

fn parse_discs(input: &String) -> Vec<Disc> {
    let mut discs = Vec::new();

    for line in input.lines() {
        let mut words = line.split_ascii_whitespace();

        words.next();
        words.next();
        words.next();

        let position_count = words.next().unwrap().parse().unwrap();

        words.next();
        words.next();
        words.next();
        words.next();
        words.next();
        words.next();
        words.next();

        let starting_position = words.next().unwrap();
        let starting_position = starting_position[..starting_position.len() - 1]
            .parse()
            .unwrap();

        discs.push(Disc {
            position_count,
            starting_position,
        })
    }

    discs
}

fn find_capsule_time(discs: &Vec<Disc>) -> usize {
    for i in 0.. {
        if discs
            .iter()
            .enumerate()
            .all(|(offset, disc)| disc.get_position(i + offset + 1) == 0)
        {
            return i;
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

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;
    let mut discs = parse_discs(&input);

    time_it(|| println!("part 1: {}", find_capsule_time(&discs)));

    discs.push(Disc {
        starting_position: 0,
        position_count: 11,
    });

    time_it(|| println!("part 2: {}", find_capsule_time(&discs)));

    Ok(())
}
