use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug)]
enum Arg {
    Val(i64),
    Reg(char),
}

impl Arg {}

impl std::str::FromStr for Arg {
    type Err = String;

    fn from_str(string: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match string.parse::<i64>() {
            Ok(value) => Ok(Self::Val(value)),
            _ => match string.chars().nth(0) {
                Some(c) => Ok(Self::Reg(c)),
                _ => Err(string.to_owned()),
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Set(Arg, Arg),
    Mul(Arg, Arg),
    Sub(Arg, Arg),
    Jnz(Arg, Arg),
}

impl std::str::FromStr for Instruction {
    type Err = String;

    fn from_str(string: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let mut words = string.split_ascii_whitespace();
        let kind = words.next();

        let arg1 = match words.next() {
            Some(word) => match word.parse() {
                Ok(arg) => arg,
                _ => return Err(string.to_owned()),
            },
            _ => return Err(string.to_owned()),
        };

        let arg2 = match words.next() {
            Some(word) => match word.parse() {
                Ok(value) => value,
                _ => return Err(string.to_owned()),
            },
            _ => return Err(string.to_owned()),
        };

        match kind {
            Some("set") => Ok(Self::Set(arg1, arg2)),
            Some("sub") => Ok(Self::Sub(arg1, arg2)),
            Some("mul") => Ok(Self::Mul(arg1, arg2)),
            Some("jnz") => Ok(Self::Jnz(arg1, arg2)),
            _ => Err(string.to_owned()),
        }
    }
}

struct Program {
    pc: usize,
    registers: HashMap<char, i64>,
    instructions: Vec<Instruction>,
    mul_invocations: i32,
}

impl Program {
    fn new(instructions: Vec<Instruction>, debug_mode: bool) -> Self {
        let mut registers: HashMap<_, _> = ('a'..='h').map(|ch| (ch, 0)).collect();

        if debug_mode {
            *registers.entry('a').or_default() = 1;
        }

        Self {
            pc: 0,
            mul_invocations: 0,
            registers,
            instructions,
        }
    }

    fn jump(&mut self, amount: i64) {
        match amount.cmp(&0) {
            Ordering::Equal => (),
            Ordering::Greater => self.pc += amount as usize,
            Ordering::Less => self.pc -= amount.wrapping_abs() as usize,
        }
    }

    fn get_register_mut(&mut self, name: char) -> &mut i64 {
        self.registers.entry(name).or_default()
    }

    fn get_register(&mut self, name: char) -> &i64 {
        self.registers.entry(name).or_default()
    }

    fn run(&mut self) {
        while let Some(&instruction) = self.instructions.get(self.pc) {
            // println!("{:?}", self.registers);
            match instruction {
                Instruction::Set(Arg::Reg(register), Arg::Val(number)) => {
                    *self.get_register_mut(register) = number
                }
                Instruction::Set(Arg::Reg(dest), Arg::Reg(src)) => {
                    let number = *self.get_register(src);
                    *self.get_register_mut(dest) = number
                }
                Instruction::Sub(Arg::Reg(register), Arg::Val(number)) => {
                    *self.get_register_mut(register) -= number
                }
                Instruction::Sub(Arg::Reg(register), Arg::Reg(value)) => {
                    let value = *self.get_register(value);
                    *self.get_register_mut(register) -= value
                }
                Instruction::Mul(Arg::Reg(register), Arg::Val(value)) => {
                    self.mul_invocations += 1;
                    *self.get_register_mut(register) *= value
                }
                Instruction::Mul(Arg::Reg(register), Arg::Reg(value)) => {
                    self.mul_invocations += 1;
                    let value = *self.get_register(value);
                    *self.get_register_mut(register) *= value
                }
                Instruction::Jnz(Arg::Reg(register), Arg::Val(amount)) => {
                    if *self.get_register(register) != 0 {
                        self.jump(amount);
                        continue;
                    }
                }
                Instruction::Jnz(Arg::Reg(register), Arg::Reg(value)) => {
                    if *self.get_register(register) != 0 {
                        let amount = *self.get_register(value);
                        self.jump(amount);
                        continue;
                    }
                }
                Instruction::Jnz(Arg::Val(cmp), Arg::Val(amount)) => {
                    if cmp != 0 {
                        self.jump(amount);
                        continue;
                    }
                }
                _ => unreachable!("{:?}", instruction),
            }

            self.pc += 1
        }
    }
}

fn part_one(input: &str) -> i32 {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut program = Program::new(instructions, false);

    program.run();

    program.mul_invocations
}

fn is_prime(number: i64) -> bool {
    for d in 2..(number as f64).sqrt() as i64 {
        if number % d == 0 {
            return false;
        }
    }

    true
}

fn part_two(input: &str) -> i32 {
    let mut instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();

    instructions.drain(8..);

    let mut program = Program::new(instructions, true);

    program.run();

    let start = *program.registers.get(&'b').unwrap();
    let end = *program.registers.get(&'c').unwrap();

    (start..=end)
        .step_by(17)
        .map(|n| if is_prime(n) { 0 } else { 1 })
        .sum()
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
