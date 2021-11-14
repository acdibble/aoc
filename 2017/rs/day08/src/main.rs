use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

type Registers<'a> = HashMap<&'a str, i32>;

enum ComparisonKind {
    Lte,
    Lt,
    Ne,
    Gt,
    Gte,
    Eq,
}

struct Comparison<'a> {
    register: &'a str,
    value: i32,
    kind: ComparisonKind,
}

impl<'a> Comparison<'a> {
    fn from(comparison: &'a str, register: &'a str, value: i32) -> Self {
        match comparison {
            "<=" => Comparison {
                register,
                value,
                kind: ComparisonKind::Lte,
            },
            "<" => Comparison {
                register,
                value,
                kind: ComparisonKind::Lt,
            },
            "!=" => Comparison {
                register,
                value,
                kind: ComparisonKind::Ne,
            },
            ">" => Comparison {
                register,
                value,
                kind: ComparisonKind::Gt,
            },
            ">=" => Comparison {
                register,
                value,
                kind: ComparisonKind::Gte,
            },
            "==" => Comparison {
                register,
                value,
                kind: ComparisonKind::Eq,
            },
            _ => unreachable!(),
        }
    }

    fn evaluate(&self, registers: &Registers<'a>) -> bool {
        let register_value = *registers.get(self.register).unwrap_or(&0);
        match self.kind {
            ComparisonKind::Eq => register_value == self.value,
            ComparisonKind::Ne => register_value != self.value,
            ComparisonKind::Lt => register_value < self.value,
            ComparisonKind::Lte => register_value <= self.value,
            ComparisonKind::Gt => register_value > self.value,
            ComparisonKind::Gte => register_value >= self.value,
        }
    }
}

enum Instruction<'a> {
    Inc(&'a str, i32, Comparison<'a>),
    Dec(&'a str, i32, Comparison<'a>),
}

fn parse_instruction(line: &str) -> Instruction {
    let mut words = line.split_ascii_whitespace();

    let register = words.next().unwrap();
    let action = words.next().unwrap();
    let amount = words.next().unwrap().parse().unwrap();

    words.next();

    let register2 = words.next().unwrap();
    let comparison = words.next().unwrap();
    let value = words.next().unwrap().parse().unwrap();

    match action {
        "inc" => Instruction::Inc(
            register,
            amount,
            Comparison::from(comparison, register2, value),
        ),
        "dec" => Instruction::Dec(
            register,
            amount,
            Comparison::from(comparison, register2, value),
        ),
        _ => unreachable!(),
    }
}

fn part_one(input: &str) -> (i32, i32) {
    let instructions: Vec<_> = input.lines().map(parse_instruction).collect();
    let mut registers: Registers = HashMap::new();
    let mut all_time_max = i32::MIN;

    for instruction in instructions {
        match instruction {
            Instruction::Inc(register, amount, comparison) => {
                if comparison.evaluate(&registers) {
                    let entry = registers.entry(register).or_default();
                    *entry += amount;
                    all_time_max = all_time_max.max(*entry);
                }
            }
            Instruction::Dec(register, amount, comparison) => {
                if comparison.evaluate(&registers) {
                    let entry = registers.entry(register).or_default();
                    *entry -= amount;
                    all_time_max = all_time_max.max(*entry);
                }
            }
        }
    }

    (*registers.values().max().unwrap(), all_time_max)
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

    time_it(|| println!("parts (1, 2): {:?}", part_one(&input)));

    Ok(())
}
