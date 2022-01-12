use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
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

    fn from_str(string: &str) -> Result<Self, Self::Err> {
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
    Snd(Arg),
    Set(Arg, Arg),
    Add(Arg, Arg),
    Mul(Arg, Arg),
    Mod(Arg, Arg),
    Rcv(Arg),
    Jgz(Arg, Arg),
}

impl std::str::FromStr for Instruction {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
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
                Ok(value) => Some(value),
                _ => return Err(string.to_owned()),
            },
            None => None,
        };

        match kind {
            Some("snd") => Ok(Self::Snd(arg1)),
            Some("set") => Ok(Self::Set(arg1, arg2.unwrap())),
            Some("add") => Ok(Self::Add(arg1, arg2.unwrap())),
            Some("mul") => Ok(Self::Mul(arg1, arg2.unwrap())),
            Some("mod") => Ok(Self::Mod(arg1, arg2.unwrap())),
            Some("rcv") => Ok(Self::Rcv(arg1)),
            Some("jgz") => Ok(Self::Jgz(arg1, arg2.unwrap())),
            _ => Err(string.to_owned()),
        }
    }
}

struct Program {
    pc: usize,
    registers: HashMap<char, i64>,
    previous_sound: Option<i64>,
    instructions: Vec<Instruction>,
    inbox: Option<VecDeque<i64>>,
    outbox: Option<VecDeque<i64>>,
    values_emitted: i32,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            pc: 0,
            values_emitted: 0,
            registers: HashMap::new(),
            previous_sound: None,
            inbox: None,
            outbox: None,
            instructions,
        }
    }

    fn with_io(instructions: Vec<Instruction>, id: i64) -> Self {
        let mut program = Self::new(instructions);
        program.inbox = Some(VecDeque::new());
        program.outbox = Some(VecDeque::new());
        program.registers.entry('p').or_insert(id);
        program
    }

    fn jump(&mut self, arg1: Arg, arg2: Arg) -> bool {
        let to_compare = match arg1 {
            Arg::Reg(ch) => *self.get_register(ch),
            Arg::Val(v) => v,
        };

        if to_compare <= 0 {
            return false;
        }

        let amount = match arg2 {
            Arg::Reg(ch) => *self.get_register(ch),
            Arg::Val(v) => v,
        };

        match amount.cmp(&0) {
            Ordering::Equal => (),
            Ordering::Greater => self.pc += amount as usize,
            Ordering::Less => self.pc -= amount.wrapping_abs() as usize,
        }

        true
    }

    fn send(&mut self, value: i64) {
        if let Some(queue) = &mut self.outbox {
            queue.push_back(value);
        } else {
            self.previous_sound = Some(value)
        }

        self.values_emitted += 1;
    }

    fn is_empty(&self) -> bool {
        match &self.inbox {
            Some(queue) => queue.is_empty(),
            _ => unreachable!(),
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
            match instruction {
                Instruction::Add(Arg::Reg(register), Arg::Val(number)) => {
                    *self.get_register_mut(register) += number
                }
                Instruction::Add(Arg::Reg(register), Arg::Reg(value)) => {
                    let value = *self.get_register(value);
                    *self.get_register_mut(register) += value
                }
                Instruction::Set(Arg::Reg(register), Arg::Val(number)) => {
                    *self.get_register_mut(register) = number
                }
                Instruction::Set(Arg::Reg(dest), Arg::Reg(src)) => {
                    let number = *self.get_register(src);
                    *self.get_register_mut(dest) = number
                }
                Instruction::Mul(Arg::Reg(register), Arg::Val(value)) => {
                    *self.get_register_mut(register) *= value
                }
                Instruction::Mul(Arg::Reg(register), Arg::Reg(value)) => {
                    let value = *self.get_register(value);
                    *self.get_register_mut(register) *= value
                }
                Instruction::Mod(Arg::Reg(register), Arg::Val(number)) => {
                    *self.get_register_mut(register) %= number
                }
                Instruction::Mod(Arg::Reg(register), Arg::Reg(value)) => {
                    let number = *self.get_register(value);
                    *self.get_register_mut(register) %= number
                }
                Instruction::Snd(Arg::Val(value)) => self.send(value),
                Instruction::Snd(Arg::Reg(register)) => {
                    let value = *self.get_register(register);
                    self.send(value)
                }
                Instruction::Rcv(Arg::Reg(register)) => {
                    if let Some(queue) = &mut self.inbox {
                        match queue.pop_front() {
                            Some(value) => *self.get_register_mut(register) = value,
                            _ => break,
                        }
                    } else if *self.get_register(register) != 0 {
                        break;
                    }
                }
                Instruction::Jgz(arg1, arg2) => {
                    if self.jump(arg1, arg2) {
                        continue;
                    }
                }
                _ => unreachable!("{:?}", instruction),
            }

            self.pc += 1
        }
    }
}

fn part_one(input: &str) -> i64 {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut program = Program::new(instructions);

    program.run();

    program.previous_sound.unwrap()
}

fn part_two(input: &str) -> i32 {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut program_0 = Program::with_io(instructions.clone(), 0);
    let mut program_1 = Program::with_io(instructions, 1);

    let mut should_run = true;
    while should_run {
        program_0.run();
        std::mem::swap(&mut program_0.outbox, &mut program_1.inbox);
        should_run = !program_1.is_empty();

        program_1.run();
        std::mem::swap(&mut program_1.outbox, &mut program_0.inbox);
        should_run |= !program_0.is_empty();
    }

    program_1.values_emitted
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
