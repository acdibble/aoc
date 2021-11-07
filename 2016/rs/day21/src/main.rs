use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    Reverse(usize, usize),
    Rotate(Direction, usize),
    Move(usize, usize),
    RotateAt(char),
}

impl Instruction {
    fn from(string: &str) -> Self {
        let mut words = string.split_ascii_whitespace();

        let instruction = match words.next() {
            Some("swap") => match words.next() {
                Some("position") => {
                    let source = words.next().unwrap().parse().unwrap();
                    words.next();
                    words.next();
                    let dest = words.next().unwrap().parse().unwrap();
                    Instruction::SwapPosition(source, dest)
                }
                Some("letter") => {
                    let a = words.next().unwrap().chars().nth(0).unwrap();
                    words.next();
                    words.next();
                    let b = words.next().unwrap().chars().nth(0).unwrap();
                    Instruction::SwapLetter(a, b)
                }
                _ => unreachable!(),
            },
            Some("reverse") => {
                words.next();
                let start = words.next().unwrap().parse().unwrap();
                words.next();
                let end = words.next().unwrap().parse().unwrap();
                Instruction::Reverse(start, end)
            }
            Some("rotate") => match words.next() {
                Some("based") => {
                    words.next();
                    words.next();
                    words.next();
                    words.next();
                    Instruction::RotateAt(words.next().unwrap().chars().nth(0).unwrap())
                }
                Some("left") => {
                    Instruction::Rotate(Direction::Left, words.next().unwrap().parse().unwrap())
                }
                Some("right") => {
                    Instruction::Rotate(Direction::Right, words.next().unwrap().parse().unwrap())
                }
                _ => unreachable!(),
            },
            Some("move") => {
                words.next();
                let source = words.next().unwrap().parse().unwrap();
                words.next();
                words.next();
                let dest = words.next().unwrap().parse().unwrap();
                Instruction::Move(source, dest)
            }
            _ => unreachable!(),
        };

        instruction
    }
}

fn calculate_rotations(index: usize, length: usize) -> usize {
    (1 + index + if index >= 4 { 1 } else { 0 }) % length
}

fn part_one(instructions: &Vec<Instruction>, password: &str) -> String {
    let mut output: VecDeque<char> = password.chars().collect();

    let rotations: Vec<_> = (0..output.len())
        .map(|n| calculate_rotations(n, output.len()))
        .collect();

    for instruction in instructions {
        match instruction {
            Instruction::SwapPosition(source, dest) => output.swap(*source, *dest),
            Instruction::SwapLetter(a, b) => {
                let mut source = None;
                let mut dest = None;
                for (index, c) in output.iter().enumerate() {
                    if *a == *c {
                        source = Some(index);
                    } else if *b == *c {
                        dest = Some(index);
                    }
                    if source.is_some() && dest.is_some() {
                        break;
                    }
                }
                output.swap(source.unwrap(), dest.unwrap())
            }
            Instruction::Reverse(start, end) => {
                for i in 0..(*end - *start) / 2 + 1 {
                    output.swap(*start + i, *end - i);
                }
            }
            Instruction::Rotate(direction, amount) => match direction {
                Direction::Left => {
                    for _ in 0..*amount {
                        let temp = output.pop_front().unwrap();
                        output.push_back(temp);
                    }
                }
                Direction::Right => {
                    for _ in 0..*amount {
                        let temp = output.pop_back().unwrap();
                        output.push_front(temp);
                    }
                }
            },
            Instruction::Move(source, dest) => {
                let temp = output.remove(*source).unwrap();
                output.insert(*dest, temp);
            }
            Instruction::RotateAt(c) => {
                let index = output.iter().position(|letter| letter == c).unwrap();
                for _ in 0..rotations[index] {
                    let temp = output.pop_back().unwrap();
                    output.push_front(temp);
                }
            }
        }
    }

    output.into_iter().collect()
}

fn part_two(instructions: &Vec<Instruction>, password: &str) -> String {
    let mut output: VecDeque<char> = password.chars().collect();

    let mut inverses: Vec<_> = (0..output.len())
        .map(|n| {
            let rotations = calculate_rotations(n, output.len());
            let new_position = (n + rotations) % output.len();
            (new_position, rotations)
        })
        .collect();
    inverses.sort_by(|a, b| a.0.cmp(&b.0));
    let inverses: Vec<_> = inverses
        .into_iter()
        .map(|(_, rotations)| rotations)
        .collect();

    for instruction in instructions.iter().rev() {
        match instruction {
            Instruction::SwapPosition(source, dest) => output.swap(*dest, *source),
            Instruction::SwapLetter(a, b) => {
                let mut source = None;
                let mut dest = None;
                for (index, c) in output.iter().enumerate() {
                    if *a == *c {
                        source = Some(index);
                    } else if *b == *c {
                        dest = Some(index);
                    }
                    if source.is_some() && dest.is_some() {
                        break;
                    }
                }
                output.swap(source.unwrap(), dest.unwrap())
            }
            Instruction::Reverse(start, end) => {
                for i in 0..(*end - *start) / 2 + 1 {
                    output.swap(*start + i, *end - i);
                }
            }
            Instruction::Rotate(direction, amount) => match direction {
                Direction::Left => {
                    for _ in 0..*amount {
                        let temp = output.pop_back().unwrap();
                        output.push_front(temp);
                    }
                }
                Direction::Right => {
                    for _ in 0..*amount {
                        let temp = output.pop_front().unwrap();
                        output.push_back(temp);
                    }
                }
            },
            Instruction::Move(source, dest) => {
                let temp = output.remove(*dest).unwrap();
                output.insert(*source, temp);
            }
            Instruction::RotateAt(c) => {
                let index = output.iter().position(|letter| letter == c).unwrap();
                let rotations = inverses[index];
                for _ in 0..rotations {
                    let temp = output.pop_front().unwrap();
                    output.push_back(temp);
                }
            }
        }
    }

    output.into_iter().collect()
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
    let instructions = input.lines().map(Instruction::from).collect();

    time_it(|| println!("part 1: {}", part_one(&instructions, "abcdefgh")));
    time_it(|| println!("part 2: {}", part_two(&instructions, "fbgdceah")));

    Ok(())
}
