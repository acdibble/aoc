use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::time::SystemTime;

fn penultimate_char(line: &str) -> char {
    line.chars().nth_back(1).unwrap()
}

fn parse_preamble(string: &str) -> (char, i32) {
    let mut lines = string.lines();

    let state = penultimate_char(lines.next().unwrap());

    let steps = lines
        .next()
        .unwrap()
        .chars()
        .filter(|ch| char::is_numeric(*ch))
        .collect::<String>()
        .parse()
        .unwrap();

    (state, steps)
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum Value {
    Zero,
    One,
}

#[derive(Debug)]
struct Step {
    state: char,
    if_0: (Value, Direction, char),
    if_1: (Value, Direction, char),
}

fn parse_substep(lines: &mut std::str::Lines) -> (Value, Direction, char) {
    lines.next();
    let value = if lines.next().unwrap().ends_with("1.") {
        Value::One
    } else {
        Value::Zero
    };

    let direction = if lines.next().unwrap().ends_with("right.") {
        Direction::Right
    } else {
        Direction::Left
    };

    let next_state = penultimate_char(lines.next().unwrap());

    (value, direction, next_state)
}

fn parse_step(string: &str) -> Step {
    let mut lines = string.lines();

    let state = penultimate_char(lines.next().unwrap());

    Step {
        state,
        if_0: parse_substep(&mut lines),
        if_1: parse_substep(&mut lines),
    }
}

fn write_file(beginning_state: char, checksum: i32, steps: Vec<Step>) -> std::io::Result<()> {
    let path = env::current_dir()?.join("src").join("turing.rs");
    let mut out = fs::File::create(path)?;

    out.write(
        b"use std::collections::HashMap;\n
const ITERATIONS: i32 = ",
    )?;
    out.write(checksum.to_string().as_bytes())?;
    out.write(
        b";

enum State {
",
    )?;

    for step in &steps {
        out.write(b"    ")?;
        out.write(&[step.state as u8])?;
        out.write(b",\n")?;
    }
    out.write(
        b"}

#[derive(Clone, Copy)]
enum Value {
    Zero,
    One,
}

impl Default for Value {
    fn default() -> Self {
        Self::Zero
    }
}

enum Direction {
    Left,
    Right,
}

struct Tape {
    storage: HashMap<i32, Value>,
    position: i32,
}

impl Tape {
    fn new() -> Self {
        Self {
            position: 0,
            storage: HashMap::new(),
        }
    }

    fn move_right(&mut self) {
        self.position += 1;
    }

    fn move_left(&mut self) {
        self.position -= 1;
    }

    fn write_value(&mut self, value: Value) {
        *self.storage.entry(self.position).or_default() = value;
    }

    fn current_value(&mut self) -> Value {
        *self.storage.entry(self.position).or_default()
    }

    fn checksum(&self) -> i32 {
        self.storage
            .values()
            .map(|value| match value {
                Value::Zero => 0,
                Value::One => 1,
            })
            .sum()
    }
}

fn main() {
    let mut tape = Tape::new();
    let mut current_state = State::",
    )?;
    out.write(&[beginning_state as u8])?;
    out.write(
        b";

    for _ in 0..ITERATIONS {
        let (new_value, direction, new_state) = match (current_state, tape.current_value()) {\n",
    )?;

    for step in &steps {
        out.write(b"            (State::")?;
        out.write(&[step.state as u8])?;
        out.write(b", Value::Zero) => ")?;
        out.write(
            format!(
                "(Value::{:?}, Direction::{:?}, State::{}),\n",
                step.if_0.0, step.if_0.1, step.if_0.2
            )
            .as_bytes(),
        )?;
        out.write(b"            (State::")?;
        out.write(&[step.state as u8])?;
        out.write(b", Value::One) => ")?;
        out.write(
            format!(
                "(Value::{:?}, Direction::{:?}, State::{}),\n",
                step.if_1.0, step.if_1.1, step.if_1.2
            )
            .as_bytes(),
        )?;
    }

    out.write(
        b"        };

        tape.write_value(new_value);
        match direction {
            Direction::Right => tape.move_right(),
            Direction::Left => tape.move_left(),
        }
        current_state = new_state;
    }

    println!(\"{}\", tape.checksum());
}
",
    )?;

    Ok(())
}

fn part_one(input: &str) -> i32 {
    let mut parts = input.split("\n\n");

    let (beginning_state, checksum) = parse_preamble(parts.next().unwrap());

    let steps: Vec<Step> = parts.map(|p| parse_step(p)).collect();

    write_file(beginning_state, checksum, steps).expect("failed to write file");

    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("turing")
        .arg("--release")
        .output()
        .expect("failed to run cargo");

    String::from_utf8(output.stdout)
        .unwrap()
        .trim()
        .parse()
        .unwrap()
}

fn time_it<F, T>(fun: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();
    let result = fun();
    println!("Time elapsed: {} ms", start.elapsed().unwrap().as_millis());
    result
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("part 1: {}", part_one(&input)));

    Ok(())
}
