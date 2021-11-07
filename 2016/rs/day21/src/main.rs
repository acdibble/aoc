use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

impl std::ops::Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::Left => Direction::Right,
            _ => Direction::Left,
        }
    }
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

struct Password {
    buffer: VecDeque<char>,
    rotations: Vec<usize>,
}

impl Password {
    fn from(string: &str, rotations: Option<Vec<usize>>) -> Self {
        Self {
            buffer: string.chars().collect(),
            rotations: rotations.unwrap_or_else(|| {
                (0..string.len())
                    .map(|n| calculate_rotations(n, string.len()))
                    .collect()
            }),
        }
    }

    fn swap_chars(&mut self, a: char, b: char) {
        let mut source = None;
        let mut dest = None;
        for (index, c) in self.buffer.iter().enumerate() {
            if a == *c {
                source = Some(index);
            } else if b == *c {
                dest = Some(index);
            }

            match (source, dest) {
                (Some(source), Some(dest)) => {
                    self.buffer.swap(source, dest);
                    return;
                }
                _ => (),
            }
        }
    }

    fn swap_indices(&mut self, a: usize, b: usize) {
        self.buffer.swap(a, b);
    }

    fn reverse_range(&mut self, start: usize, end: usize) {
        for i in 0..(end - start) / 2 + 1 {
            self.buffer.swap(start + i, end - i);
        }
    }

    fn rotate(&mut self, direction: Direction, amount: usize) {
        let amount = match direction {
            Direction::Right => self.buffer.len() - amount,
            _ => amount,
        };
        for _ in 0..amount {
            let temp = self.buffer.pop_front().unwrap();
            self.buffer.push_back(temp);
        }
    }

    fn move_char(&mut self, src: usize, dest: usize) {
        let temp = self.buffer.remove(src).unwrap();
        self.buffer.insert(dest, temp);
    }

    fn rotate_char(&mut self, direction: Direction, c: char) {
        let index = self.buffer.iter().position(|letter| *letter == c).unwrap();
        let amount = self.rotations[index];
        self.rotate(direction, amount)
    }

    fn to_string(&self) -> String {
        self.buffer.iter().collect()
    }
}

fn part_one(instructions: &Vec<Instruction>, password: &str) -> String {
    let mut password = Password::from(password, None);

    for instruction in instructions {
        match instruction {
            Instruction::SwapPosition(a, b) => password.swap_indices(*a, *b),
            Instruction::SwapLetter(a, b) => password.swap_chars(*a, *b),
            Instruction::Reverse(start, end) => password.reverse_range(*start, *end),
            Instruction::Rotate(direction, amount) => password.rotate(*direction, *amount),
            Instruction::Move(source, dest) => password.move_char(*source, *dest),
            Instruction::RotateAt(c) => password.rotate_char(Direction::Right, *c),
        }
    }

    password.to_string()
}

fn part_two(instructions: &Vec<Instruction>, password: &str) -> String {
    let mut inverses: Vec<_> = (0..password.len())
        .map(|n| {
            let rotations = calculate_rotations(n, password.len());
            let new_position = (n + rotations) % password.len();
            (new_position, rotations)
        })
        .collect();
    inverses.sort_by(|a, b| a.0.cmp(&b.0));

    let mut password = Password::from(
        password,
        Some(
            inverses
                .into_iter()
                .map(|(_, rotations)| rotations)
                .collect(),
        ),
    );

    for instruction in instructions.iter().rev() {
        match instruction {
            Instruction::SwapPosition(a, b) => password.swap_indices(*a, *b),
            Instruction::SwapLetter(a, b) => password.swap_chars(*a, *b),
            Instruction::Reverse(start, end) => password.reverse_range(*start, *end),
            Instruction::Rotate(direction, amount) => password.rotate(!*direction, *amount),
            Instruction::Move(src, dest) => password.move_char(*dest, *src),
            Instruction::RotateAt(c) => password.rotate_char(Direction::Left, *c),
        }
    }

    password.to_string()
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
