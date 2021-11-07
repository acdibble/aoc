use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
struct Elf {
    id: usize,
    has_present: bool,
}

fn part_one(elf_count: usize) -> usize {
    let mut queue = (1..=elf_count)
        .map(|id| Elf {
            id,
            has_present: true,
        })
        .collect::<VecDeque<_>>();

    let mut current = queue.pop_front().unwrap();
    let mut next = queue.pop_front().unwrap();

    while queue.len() != 1 {
        if current.has_present {
            next.has_present = false;
            queue.push_back(current);
        }

        current = next;
        next = queue.pop_front().unwrap();
    }

    queue[0].id
}

fn part_two(elf_count: usize) -> usize {
    let mut front_half = VecDeque::with_capacity(elf_count / 2);
    let mut back_half = VecDeque::with_capacity(elf_count / 2);

    for i in 1..=elf_count {
        if i <= elf_count / 2 {
            front_half.push_back(i);
        } else {
            back_half.push_back(i);
        }
    }

    while front_half.len() != 1 {
        let current = front_half.pop_front().unwrap();
        if front_half.len() == back_half.len() {
            front_half.pop_back();
        } else {
            back_half.pop_front();
        }

        back_half.push_back(current);
        front_half.push_back(back_half.pop_front().unwrap());
    }

    front_half[0]
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
