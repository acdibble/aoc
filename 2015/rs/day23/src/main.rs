use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

enum Register {
    A,
    B,
}

impl Register {
    fn from(string: &str) -> Self {
        match string {
            "a" | "a," => Self::A,
            "b" | "b," => Self::B,
            _ => unreachable!(),
        }
    }
}

enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpIfEven(Register, i32),
    JumpIfOne(Register, i32),
}

struct Computer {
    a: u32,
    b: u32,
    ip: usize,
}

impl Computer {
    fn new() -> Self {
        Self { a: 0, b: 0, ip: 0 }
    }

    fn register_for(&mut self, register: &Register) -> &mut u32 {
        match register {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }

    fn process(&mut self, instruction: &Instruction) {
        let increment = match instruction {
            Instruction::Half(register) => {
                *self.register_for(register) /= 2;
                1
            }
            Instruction::Triple(register) => {
                *self.register_for(register) *= 3;
                1
            }
            Instruction::Increment(register) => {
                *self.register_for(register) += 1;
                1
            }
            Instruction::Jump(offset) => *offset,
            Instruction::JumpIfEven(register, offset) => {
                if *self.register_for(register) % 2 == 0 {
                    *offset
                } else {
                    1
                }
            }
            Instruction::JumpIfOne(register, offset) => {
                if *self.register_for(register) == 1 {
                    *offset
                } else {
                    1
                }
            }
        };

        if increment.is_negative() {
            self.ip -= increment.wrapping_abs() as usize
        } else {
            self.ip += increment as usize
        }
    }

    fn evaluate(instructions: &Vec<Instruction>) -> u32 {
        let mut computer = Self::new();

        while let Some(instruction) = instructions.get(computer.ip) {
            computer.process(instruction);
        }

        computer.b
    }
}

fn parse_instructions(string: String) -> Vec<Instruction> {
    let mut output = Vec::new();

    for line in string.lines() {
        let mut it = line.split_ascii_whitespace();

        let instruction = match it.next().unwrap() {
            inst @ ("hlf" | "tpl" | "inc") => {
                let register = Register::from(it.next().unwrap());

                match inst {
                    "hlf" => Instruction::Half(register),
                    "tpl" => Instruction::Triple(register),
                    "inc" => Instruction::Increment(register),
                    _ => unreachable!(),
                }
            }
            "jmp" => {
                let offset: i32 = it.next().unwrap().parse().unwrap();

                Instruction::Jump(offset)
            }
            inst @ ("jie" | "jio") => {
                let register = Register::from(it.next().unwrap());
                let offset: i32 = it.next().unwrap().parse().unwrap();

                match inst {
                    "jie" => Instruction::JumpIfEven(register, offset),
                    "jio" => Instruction::JumpIfOne(register, offset),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        };

        output.push(instruction)
    }

    output
}

fn time_it<F>(fun: F)
where
    F: Fn() -> (),
{
    let start = SystemTime::now();
    fun();
    println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros())
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    let mut instructions = parse_instructions(input);
    time_it(|| println!("part 1: {}", Computer::evaluate(&instructions)));
    instructions.insert(0, Instruction::Increment(Register::A));
    time_it(|| println!("part 2: {}", Computer::evaluate(&instructions)));

    Ok(())
}
