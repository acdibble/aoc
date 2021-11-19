use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(input: i32) -> i32 {
    let mut queue = VecDeque::from([0]);

    for i in 1..=2017 {
        for _ in 0..input + 1 {
            let temp = queue.pop_front().unwrap();
            queue.push_back(temp);
        }

        queue.push_front(i)
    }

    *queue.get(1).unwrap()
}

fn part_two(input: i32) -> i32 {
    let mut queue = VecDeque::with_capacity(50_000_001);
    queue.push_front(0);

    for i in 1..=50_000_000 {
        for _ in 0..input + 1 {
            let temp = queue.pop_front().unwrap();
            queue.push_back(temp);
        }

        queue.push_front(i)
    }

    let index = queue.iter().position(|&v| v == 0).unwrap();

    match queue.get(index + 1) {
        Some(&value) => value,
        _ => panic!(),
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

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?.parse().unwrap();

    time_it(|| println!("part 1: {}", part_one(input)));
    time_it(|| println!("part 2: {}", part_two(input)));

    Ok(())
}
