use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;
use std::path::Path;
use std::str::Split;

#[derive(Debug, Copy, Clone)]
enum Instruction<'a> {
    Not(&'a str, &'a str),
    OneAnd(&'a str, &'a str),
    LeftShift(&'a str, usize, &'a str),
    RightShift(&'a str, usize, &'a str),
    And(&'a str, &'a str, &'a str),
    Or(&'a str, &'a str, &'a str),
    Literal(u16, &'a str),
    Set(&'a str, &'a str),
}

fn not_instruction<'a>(mut parts: Split<'a, char>) -> Option<Instruction<'a>> {
    let arg = parts.next()?;
    assert!(parts.next()? == "->");
    let target = parts.next()?;

    Some(Instruction::Not(arg, target))
}

fn one_and_instruction<'a>(mut parts: Split<'a, char>) -> Option<Instruction<'a>> {
    assert!(parts.next()? == "AND");
    let arg = parts.next()?;
    assert!(parts.next()? == "->");
    let target = parts.next()?;

    Some(Instruction::OneAnd(arg, target))
}

fn literal_instruction<'a>(
    operand1: &'a str,
    mut parts: Split<'a, char>,
) -> Option<Instruction<'a>> {
    let target = parts.next()?;

    Some(match operand1.parse::<u16>() {
        Ok(value) => Instruction::Literal(value, target),
        _ => Instruction::Set(operand1, target),
    })
}

fn binary_instruction<'a>(
    operand1: &'a str,
    mut parts: Split<'a, char>,
) -> Option<Instruction<'a>> {
    let op = parts.next()?;
    if op == "->" {
        return literal_instruction(operand1, parts);
    }
    let operand2 = parts.next()?;
    assert!(parts.next()? == "->");
    let target = parts.next()?;

    Some(match op {
        "LSHIFT" => {
            let count: usize = operand2.parse().unwrap();
            Instruction::LeftShift(operand1, count, target)
        }
        "RSHIFT" => {
            let count: usize = operand2.parse().unwrap();
            Instruction::RightShift(operand1, count, target)
        }
        "OR" => Instruction::Or(operand1, operand2, target),
        "AND" => Instruction::And(operand1, operand2, target),
        _ => unreachable!(op),
    })
}

fn evaluate_wires(input: &VecDeque<Instruction>) -> u16 {
    let mut instructions: VecDeque<Instruction> = input.clone();
    let mut registers = HashMap::<&str, u16>::new();

    while let Some(inst) = instructions.pop_front() {
        let handled = match &inst {
            Instruction::Literal(value, target) => {
                registers.insert(*target, *value);
                true
            }
            Instruction::Set(operand, target) => {
                if let Some(&value) = registers.get(*operand) {
                    registers.insert(*target, value);
                    true
                } else {
                    false
                }
            }
            Instruction::Not(operand, target) => {
                if let Some(&value) = registers.get(*operand) {
                    registers.insert(*target, !value);
                    true
                } else {
                    false
                }
            }
            Instruction::OneAnd(operand, target) => {
                if let Some(&value) = registers.get(*operand) {
                    registers.insert(*target, 1 & value);
                    true
                } else {
                    false
                }
            }
            Instruction::LeftShift(operand, amount, target) => {
                if let Some(&value) = registers.get(*operand) {
                    registers.insert(*target, value << *amount);
                    true
                } else {
                    false
                }
            }
            Instruction::RightShift(operand, amount, target) => {
                if let Some(&value) = registers.get(*operand) {
                    registers.insert(*target, value >> *amount);
                    true
                } else {
                    false
                }
            }
            Instruction::And(operand1, operand2, target) => {
                match (registers.get(*operand1), registers.get(*operand2)) {
                    (Some(&value1), Some(&value2)) => {
                        registers.insert(*target, value1 & value2);
                        true
                    }
                    _ => false,
                }
            }
            Instruction::Or(operand1, operand2, target) => {
                match (registers.get(*operand1), registers.get(*operand2)) {
                    (Some(&value1), Some(&value2)) => {
                        registers.insert(*target, value1 | value2);
                        true
                    }
                    _ => false,
                }
            }
        };

        if !handled {
            instructions.push_back(inst);
        }
    }

    *registers.get("a").unwrap()
}

fn part_one(input: &VecDeque<Instruction>) -> u16 {
    evaluate_wires(input)
}

fn part_two(input: &mut VecDeque<Instruction>, result: u16) -> u16 {
    loop {
        let inst = input.pop_front().unwrap();
        if matches!(inst, Instruction::Literal(_, "b")) {
            input.push_back(Instruction::Literal(result, "b"));
            break;
        } else {
            input.push_back(inst);
        }
    }
    evaluate_wires(input)
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    let mut parts = line.split(' ');

    match parts.next()? {
        "NOT" => not_instruction(parts),
        "1" => one_and_instruction(parts),
        operand1 => binary_instruction(operand1, parts),
    }
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let file = fs::read_to_string(file_path)?;
    let mut input: VecDeque<Instruction> = file
        .lines()
        .map(|line| parse_instruction(line).expect(line))
        .collect();

    let result = part_one(&input);
    println!("part 1: {}", &result);
    println!("part 2: {}", part_two(&mut input, result));

    Ok(())
}
