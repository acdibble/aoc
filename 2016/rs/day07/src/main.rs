use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn part_one(input: &String) -> i32 {
    let mut total = 0;

    let mut queue = VecDeque::new();

    let mut in_hypernet_sequence: bool;
    let mut abba_found;

    for line in input.lines() {
        queue.clear();
        in_hypernet_sequence = false;
        abba_found = false;

        for c in line.chars() {
            if queue.len() < 3 {
                queue.push_back(c);
                continue;
            }

            match c {
                '[' => {
                    in_hypernet_sequence = true;
                    queue.clear()
                }
                ']' => {
                    in_hypernet_sequence = false;
                    queue.clear()
                }
                _ => {
                    queue.push_back(c);
                    let first = *queue.get(0).unwrap();
                    let second = *queue.get(1).unwrap();
                    let third = *queue.get(2).unwrap();
                    let fourth = *queue.get(3).unwrap();
                    if first == fourth && second == third && first != second {
                        if in_hypernet_sequence {
                            abba_found = false;
                            break;
                        }
                        abba_found = true;
                    }
                    queue.pop_front();
                }
            }
        }

        if abba_found {
            total += 1;
        }
    }

    total
}

fn flip_sequence(&(a, b, _): &(char, char, char)) -> (char, char, char) {
    (b, a, b)
}

fn part_two(input: &String) -> i32 {
    let mut total = 0;

    let mut hypernet_sequences = Vec::new();
    let mut supernet_sequences = Vec::new();
    let mut in_hypernet_sequence: bool;

    let mut queue = VecDeque::new();

    for line in input.lines() {
        in_hypernet_sequence = false;
        hypernet_sequences.clear();
        supernet_sequences.clear();
        queue.clear();

        for c in line.chars() {
            if queue.len() < 2 {
                queue.push_back(c);
                continue;
            }

            match c {
                '[' => {
                    in_hypernet_sequence = true;
                    queue.clear()
                }
                ']' => {
                    in_hypernet_sequence = false;
                    queue.clear()
                }
                _ => {
                    queue.push_back(c);
                    let first = *queue.get(0).unwrap();
                    let second = *queue.get(1).unwrap();
                    let third = *queue.get(2).unwrap();
                    if first == third && first != second {
                        let triple = (first, second, third);
                        if in_hypernet_sequence {
                            hypernet_sequences.push(triple);
                        } else {
                            supernet_sequences.push(triple);
                        }
                    }
                    queue.pop_front();
                }
            }
        }

        for seq in &supernet_sequences {
            let flipped = flip_sequence(seq);
            if hypernet_sequences.contains(&flipped) {
                total += 1;
                break;
            }
        }
    }

    total
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

    time_it(|| println!("part 1: {}", part_one(&input)));
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}
