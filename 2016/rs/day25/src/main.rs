use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug)]
enum Register {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Copy, Debug)]
enum Argument {
    Literal(i32),
    Register(Register),
}

impl std::str::FromStr for Argument {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "a" => Ok(Self::Register(Register::A)),
            "b" => Ok(Self::Register(Register::B)),
            "c" => Ok(Self::Register(Register::C)),
            "d" => Ok(Self::Register(Register::D)),
            _ => match string.parse::<i32>() {
                Ok(value) => Ok(Self::Literal(value)),
                _ => Err(()),
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Copy(Argument, Argument),
    Increment(Argument),
    Decrement(Argument),
    JumpIfNotZero(Argument, Argument),
    Out(Argument),
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.split_ascii_whitespace();

        let inst = match parts.next().unwrap() {
            "cpy" => Instruction::Copy(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ),
            "inc" => Instruction::Increment(parts.next().unwrap().parse().unwrap()),
            "dec" => Instruction::Decrement(parts.next().unwrap().parse().unwrap()),
            "jnz" => Instruction::JumpIfNotZero(
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ),
            "out" => Instruction::Out(parts.next().unwrap().parse().unwrap()),
            _ => unreachable!(),
        };

        Ok(inst)
    }
}

fn parse_instructions(input: &String) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

struct Computer {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    pc: usize,

    instructions: Vec<Instruction>,
    output: Vec<i32>,
}

impl Computer {
    fn new(input: &String) -> Self {
        let instructions = parse_instructions(input);
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            pc: 0,
            instructions,
            output: Vec::with_capacity(10),
        }
    }

    fn reset(&mut self, a: i32) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.d = 0;
        self.pc = 0;
        self.output.clear();
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
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
        }
    }

    fn run(&mut self) -> i32 {
        while let Some(instruction) = self.instructions.get(self.pc) {
            match instruction {
                &Instruction::Copy(Argument::Register(src), Argument::Register(dest)) => {
                    *self.get_register_mut(dest) = *self.get_register(src)
                }
                &Instruction::Copy(Argument::Literal(value), Argument::Register(dest)) => {
                    *self.get_register_mut(dest) = value
                }
                &Instruction::Increment(Argument::Register(reg)) => {
                    *self.get_register_mut(reg) += 1
                }
                &Instruction::Decrement(Argument::Register(reg)) => {
                    *self.get_register_mut(reg) -= 1
                }
                &Instruction::JumpIfNotZero(Argument::Register(reg), Argument::Literal(offset)) => {
                    if *self.get_register(reg) != 0 {
                        if offset.is_negative() {
                            self.pc -= offset.wrapping_abs() as usize
                        } else {
                            self.pc += offset as usize
                        }

                        continue;
                    }
                }
                &Instruction::JumpIfNotZero(
                    Argument::Literal(value),
                    Argument::Literal(offset),
                ) => {
                    if value != 0 {
                        if offset.is_negative() {
                            self.pc -= offset.wrapping_abs() as usize
                        } else {
                            self.pc += offset as usize
                        }

                        continue;
                    }
                }
                &Instruction::JumpIfNotZero(Argument::Literal(value), Argument::Register(reg)) => {
                    if value != 0 {
                        let offset = *self.get_register(reg);

                        if offset.is_negative() {
                            self.pc -= offset.wrapping_abs() as usize
                        } else {
                            self.pc += offset as usize
                        }

                        continue;
                    }
                }

                &Instruction::Out(Argument::Register(reg)) => {
                    let value = *self.get_register(reg);
                    self.output.push(value);
                    break;
                }
                &Instruction::Out(Argument::Literal(value)) => {
                    self.output.push(value);
                    break;
                }
                _ => unreachable!("{:?}", instruction),
            }

            self.pc += 1;
        }

        self.a
    }
}

fn part_one(input: &String) -> i32 {
    let mut computer = Computer::new(input);

    for i in 1.. {
        computer.reset(i);

        loop {
            computer.run();
            computer.pc += 1;

            match computer.output.as_slice() {
                &[0]
                | &[0, 1]
                | &[0, 1, 0]
                | &[0, 1, 0, 1]
                | &[0, 1, 0, 1, 0]
                | &[0, 1, 0, 1, 0, 1]
                | &[0, 1, 0, 1, 0, 1, 0] => continue,
                &[0, 1, 0, 1, 0, 1, 0, 1] => return i,
                _ => break,
            }
        }
    }

    unreachable!()
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

    Ok(())
}
