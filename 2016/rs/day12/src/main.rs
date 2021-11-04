use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
}

impl std::str::FromStr for Register {
    type Err = ();

    fn from_str(string: &str) -> std::result::Result<Self, Self::Err> {
        match string {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    CopyValue(i32, Register),
    CopyRegister(Register, Register),
    Increment(Register),
    Decrement(Register),
    JumpIfNotZeroValue(i32, i32),
    JumpIfNotZero(Register, i32),
}

fn parse_instructions(input: &String) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();

        let inst = match parts.next().unwrap() {
            "cpy" => {
                let register_or_value = parts.next().unwrap();
                let destination = parts.next().unwrap().parse().unwrap();

                match register_or_value {
                    "a" => Instruction::CopyRegister(Register::A, destination),
                    "b" => Instruction::CopyRegister(Register::B, destination),
                    "c" => Instruction::CopyRegister(Register::C, destination),
                    "d" => Instruction::CopyRegister(Register::D, destination),
                    _ => Instruction::CopyValue(register_or_value.parse().unwrap(), destination),
                }
            }
            "inc" => Instruction::Increment(parts.next().unwrap().parse().unwrap()),
            "dec" => Instruction::Decrement(parts.next().unwrap().parse().unwrap()),
            "jnz" => {
                let value_or_register = parts.next().unwrap();
                let offset = parts.next().unwrap().parse().unwrap();

                if let Ok(register) = value_or_register.parse() {
                    Instruction::JumpIfNotZero(register, offset)
                } else {
                    Instruction::JumpIfNotZeroValue(value_or_register.parse().unwrap(), offset)
                }
            }
            _ => unreachable!(),
        };

        instructions.push(inst);
    }

    instructions
}

struct Computer {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    pc: usize,

    instructions: Vec<Instruction>,
}

impl Computer {
    fn new(input: &String) -> Self {
        let instructions = parse_instructions(input);

        Computer {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            pc: 0,
            instructions,
        }
    }

    fn get_register_mut(&mut self, reg: Register) -> &mut i32 {
        match reg {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
        }
    }

    fn get_register(&mut self, reg: Register) -> &i32 {
        match reg {
            Register::A => &self.a,
            Register::B => &self.b,
            Register::C => &self.c,
            Register::D => &self.d,
        }
    }

    fn run(&mut self) -> i32 {
        while let Some(instruction) = self.instructions.get(self.pc) {
            match instruction {
                &Instruction::CopyValue(value, reg) => *self.get_register_mut(reg) = value,
                &Instruction::CopyRegister(src, dest) => {
                    *self.get_register_mut(dest) = *self.get_register(src)
                }
                &Instruction::Increment(reg) => *self.get_register_mut(reg) += 1,
                &Instruction::Decrement(reg) => *self.get_register_mut(reg) -= 1,
                &Instruction::JumpIfNotZero(reg, offset) => {
                    if *self.get_register(reg) != 0 {
                        if offset.is_negative() {
                            self.pc -= offset.wrapping_abs() as usize
                        } else {
                            self.pc += offset as usize
                        }

                        continue;
                    }
                }
                &Instruction::JumpIfNotZeroValue(value, offset) => {
                    if value != 0 {
                        if offset.is_negative() {
                            self.pc -= offset.wrapping_abs() as usize
                        } else {
                            self.pc += offset as usize
                        }

                        continue;
                    }
                }
            }

            self.pc += 1;
        }

        self.a
    }
}

fn part_one(input: &String) -> i32 {
    Computer::new(input).run()
}

fn part_two(input: &String) -> i32 {
    let mut computer = Computer::new(input);
    computer.c = 1;
    computer.run()
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
