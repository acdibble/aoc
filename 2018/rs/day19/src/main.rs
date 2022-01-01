use std::{env, fs, path::Path, time::SystemTime};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl std::str::FromStr for Opcode {
    type Err = ();

    fn from_str(string: &str) -> std::result::Result<Opcode, ()> {
        match string {
            "addr" => Ok(Self::Addr),
            "addi" => Ok(Self::Addi),
            "mulr" => Ok(Self::Mulr),
            "muli" => Ok(Self::Muli),
            "banr" => Ok(Self::Banr),
            "bani" => Ok(Self::Bani),
            "borr" => Ok(Self::Borr),
            "bori" => Ok(Self::Bori),
            "setr" => Ok(Self::Setr),
            "seti" => Ok(Self::Seti),
            "gtir" => Ok(Self::Gtir),
            "gtri" => Ok(Self::Gtri),
            "gtrr" => Ok(Self::Gtrr),
            "eqir" => Ok(Self::Eqir),
            "eqri" => Ok(Self::Eqri),
            "eqrr" => Ok(Self::Eqrr),
            _ => Err(()),
        }
    }
}

struct Instruction(Opcode, usize, usize, usize);

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(line: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = line.split_ascii_whitespace();
        Ok(Self(
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        ))
    }
}

#[derive(Debug, Copy, Clone)]
struct CPU {
    registers: [usize; 6],
    ip_register: usize,
    ip: usize,
}

impl CPU {
    fn new(ip_register: usize) -> Self {
        Self {
            ip_register,
            ip: 0,
            registers: [0; 6],
        }
    }

    fn evaluate(&mut self, instruction: &Instruction) {
        macro_rules! register_a {
            () => {
                self.registers[instruction.1]
            };
        }

        macro_rules! register_b {
            () => {
                self.registers[instruction.2]
            };
        }
        macro_rules! value_a {
            () => {
                instruction.1
            };
        }

        macro_rules! value_b {
            () => {
                instruction.2
            };
        }

        self.registers[self.ip_register] = self.ip;

        let result = match instruction.0 {
            Opcode::Addr => register_a!() + register_b!(),
            Opcode::Addi => register_a!() + value_b!(),
            Opcode::Mulr => register_a!() * register_b!(),
            Opcode::Muli => register_a!() * value_b!(),
            Opcode::Banr => register_a!() & register_b!(),
            Opcode::Bani => register_a!() & value_b!(),
            Opcode::Borr => register_a!() | register_b!(),
            Opcode::Bori => register_a!() | value_b!(),
            Opcode::Setr => register_a!(),
            Opcode::Seti => value_a!(),
            Opcode::Gtir => (value_a!() > register_b!()) as usize,
            Opcode::Gtri => (register_a!() > value_b!()) as usize,
            Opcode::Gtrr => (register_a!() > register_b!()) as usize,
            Opcode::Eqir => (value_a!() == register_b!()) as usize,
            Opcode::Eqri => (register_a!() == value_b!()) as usize,
            Opcode::Eqrr => (register_a!() == register_b!()) as usize,
        };

        self.registers[instruction.3] = result;
        self.ip = self.registers[self.ip_register];
        self.ip += 1;
    }
}

fn parse_instructions(input: &str) -> (CPU, Vec<Instruction>) {
    let ip_register = input
        .lines()
        .nth(0)
        .unwrap()
        .split_ascii_whitespace()
        .nth_back(0)
        .unwrap()
        .parse()
        .unwrap();

    let instructions: Vec<_> = input
        .lines()
        .skip(1)
        .flat_map(str::parse::<Instruction>)
        .collect();

    let cpu = CPU::new(ip_register);

    (cpu, instructions)
}

fn part_one(input: &str) -> usize {
    let (mut cpu, instructions) = parse_instructions(input);

    while let Some(instruction) = instructions.get(cpu.ip) {
        cpu.evaluate(instruction);
    }

    cpu.registers[0]
}

fn part_two(input: &str) -> usize {
    let (mut cpu, instructions) = parse_instructions(input);

    cpu.registers[0] = 1;

    while cpu.ip != 1 {
        cpu.evaluate(&instructions[cpu.ip]);
    }

    let value = cpu.registers[2];

    let mut sum = 0;

    for i in 1..=(value / 2) {
        if value % i == 0 {
            sum += i;
        }
    }

    sum + value
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
